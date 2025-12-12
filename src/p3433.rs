use crate::Solution;

type Uid = usize;
type Timestamp = u32;

struct Event {
    timestamp: Timestamp,
    data: EventData,
}

impl Event {
    fn kind(&self) -> EventKind {
        self.data.kind()
    }
}

enum EventData {
    Message { mention: Mention },
    Offline { uid: Uid },
    Wake { uid: Uid },
}

impl EventData {
    fn kind(&self) -> EventKind {
        match self {
            Self::Message { .. } => EventKind::Message,
            Self::Offline { .. } => EventKind::Offline,
            Self::Wake { .. } => EventKind::Wake,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventKind {
    Wake,
    Offline,
    Message,
}

enum Mention {
    All,
    Here,
    Users(Vec<Uid>),
}

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut tasks = Vec::with_capacity(events.len() * 2);

        for event in events {
            let [kind, timestamp, data] = event.as_slice() else {
                unreachable!()
            };
            let timestamp: Timestamp = timestamp.parse().unwrap();

            match kind.as_str() {
                "MESSAGE" => {
                    let mention = match data.as_str() {
                        "ALL" => Mention::All,
                        "HERE" => Mention::Here,
                        _ => {
                            let users: Vec<Uid> = data
                                .split_ascii_whitespace()
                                .map(|token| token.strip_prefix("id").unwrap().parse().unwrap())
                                .collect();
                            Mention::Users(users)
                        }
                    };
                    let data = EventData::Message { mention };
                    let event = Event { timestamp, data };
                    tasks.push(event);
                }
                "OFFLINE" => {
                    let offline_event = Event {
                        timestamp,
                        data: EventData::Offline {
                            uid: data.parse().unwrap(),
                        },
                    };
                    let wake_event = Event {
                        timestamp: timestamp + 60,
                        data: EventData::Wake {
                            uid: data.parse().unwrap(),
                        },
                    };
                    tasks.push(offline_event);
                    tasks.push(wake_event);
                }
                _ => unreachable!(),
            };
        }

        tasks.sort_unstable_by_key(|task| (task.timestamp, task.kind()));

        let n = number_of_users as usize;
        let mut counts = vec![0; n];
        let mut actives = vec![true; n];
        let mut all_mention_count = 0;

        for task in tasks {
            match task.data {
                EventData::Wake { uid } => {
                    actives[uid] = true;
                }
                EventData::Offline { uid } => {
                    actives[uid] = false;
                }
                EventData::Message { mention } => match mention {
                    Mention::All => {
                        all_mention_count += 1;
                    }
                    Mention::Here => {
                        for (&is_active, count) in actives.iter().zip(&mut counts) {
                            if is_active {
                                *count += 1
                            }
                        }
                    }
                    Mention::Users(users) => {
                        for uid in users {
                            counts[uid] += 1;
                        }
                    }
                },
            }
        }

        for count in &mut counts {
            *count += all_mention_count;
        }

        counts
    }
}
