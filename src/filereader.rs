use nom::Producer;
use nom::IResult;
use nom::Offset;
use nom::FileProducer;
use nom::Consumer;
use nom::ConsumerState;
use nom::Move;
use nom::Input;
use types::Expr;
use parser::expr;
use std::str::from_utf8;
use std::ffi::OsString;

pub enum State {
    Beginning,
    End,
    Done,
    Error,
}

pub struct BCalcFileConsumer {
    pub c_state: ConsumerState<usize, (), Move>,
    pub state: State,
    pub last_expr: Option<Expr>,
}

impl BCalcFileConsumer {
    pub fn new() -> Self {
        BCalcFileConsumer {
            state: State::Beginning,
            c_state: ConsumerState::Continue(Move::Consume(0)),
            last_expr: None,
        }
    }
}

pub struct BCalcFileStreamer {
    file_producer: FileProducer,
    consumer: BCalcFileConsumer,
}

impl BCalcFileStreamer {
    pub fn new(file_name: OsString) -> Result<Self, String> {
        file_name
            .to_str()
            .map(|file| match FileProducer::new(file, 5000) {
                Ok(producer) => {
                    Ok(BCalcFileStreamer {
                        file_producer: producer,
                        consumer: BCalcFileConsumer::new(),
                    })
                }
                Err(_) => Err(format!("Could not create FileProducer for {:?}", file_name)),
            })
            .unwrap_or_else(|| {
                Err(format!("Could not create FileProducer for {:?}", file_name))
            })
    }
}

impl Iterator for BCalcFileStreamer {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        self.file_producer.apply(&mut self.consumer);
        match self.consumer.state {
            State::Error => None,
            State::Done => None,
            _ => {
                let result = self.consumer.last_expr.clone();
                result
            }
        }
    }
}

impl<'a> Consumer<&'a [u8], usize, (), Move> for BCalcFileConsumer {
    fn state(&self) -> &ConsumerState<usize, (), Move> {
        &self.c_state
    }

    fn handle(&mut self, input: Input<&'a [u8]>) -> &ConsumerState<usize, (), Move> {
        match self.state {
            State::Beginning => {
                let end_of_file = match input {
                    Input::Eof(_) => true,
                    _ => false,
                };
                match input {
                    Input::Empty | Input::Eof(None) => {
                        self.state = State::Done;
                        self.c_state = ConsumerState::Error(());
                    }
                    Input::Element(s1) |
                    Input::Eof(Some(s1)) => {
                        match expr(from_utf8(s1).unwrap()) {
                            IResult::Error(_) => {
                                self.state = State::End;
                            }
                            IResult::Incomplete(n) => {
                                if !end_of_file {
                                    self.c_state = ConsumerState::Continue(Move::Await(n));
                                } else {
                                    self.state = State::End;
                                }
                            }
                            IResult::Done(i, expr) => {
                                self.last_expr = Some(expr);
                                self.state = State::Beginning;
                                self.c_state =
                                    ConsumerState::Continue(Move::Consume(s1.offset(i.as_bytes())));
                            }
                        }
                    }
                }
            }
            State::End => {
                self.state = State::Done;
            }
            State::Done | State::Error => {
                self.state = State::Error;
                self.c_state = ConsumerState::Error(())
            }
        };
        &self.c_state
    }
}
