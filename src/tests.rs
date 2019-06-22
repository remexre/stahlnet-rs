use crate::{MessageTypeID, Server, TaskID};
use futures::prelude::*;
use libremexre::futures::run_and_return;
use pretty_assertions::assert_eq;
use std::time::Duration;
use tokio_timer::sleep;

#[test]
fn leak_check_queued() {
    let mut server = Server::new(
        ([0; 16], 0).into(),
        sleep(Duration::from_secs(1)).map_err(drop),
    )
    .unwrap();
    server.set_queue_timeout(Duration::from_millis(500));

    let initial_state = format!("{:?}", server);
    let task = server.spawn_task();
    for _ in 0..1000 {
        task.send_message(TaskID::rand(), MessageTypeID::PING, Vec::new())
            .unwrap();
    }
    drop(task);

    tokio::run(
        run_and_return(server)
            .map(move |(server, ())| assert_eq!(initial_state, format!("{:?}", server)))
            .map_err(|err| panic!("{}", err)),
    );
}

#[test]
fn leak_check_tasks() {
    let mut server = Server::new(
        ([0; 16], 0).into(),
        sleep(Duration::from_secs(1)).map_err(drop),
    )
    .unwrap();

    let initial_state = format!("{:?}", server);
    for _ in 0..1000 {
        drop(server.spawn_task());
    }

    let a = server.spawn_task();
    let b = server.spawn_task();
    a.send_message(b.id(), MessageTypeID::PING, Vec::new())
        .unwrap();
    drop(a);
    drop(b);

    tokio::run(
        run_and_return(server)
            .map(move |(server, ())| assert_eq!(initial_state, format!("{:?}", server)))
            .map_err(|err| panic!("{}", err)),
    );
}

#[test]
fn queued_are_queued() {
    let mut server = Server::new(
        ([0; 16], 0).into(),
        sleep(Duration::from_secs(1)).map_err(drop),
    )
    .unwrap();

    let initial_state = format!("{:?}", server);
    let task = server.spawn_task();
    for _ in 0..1000 {
        task.send_message(TaskID::rand(), MessageTypeID::PING, Vec::new())
            .unwrap();
    }
    drop(task);

    tokio::run(
        run_and_return(server)
            .map(move |(server, ())| assert_ne!(initial_state, format!("{:?}", server)))
            .map_err(|err| panic!("{}", err)),
    );
}
