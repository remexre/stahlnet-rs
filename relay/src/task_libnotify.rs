use libnotify::Notification;
use log::{error, info, warn};
use stahlnet::{Error, MessageTypeID, TaskHandle};
use tokio::prelude::*;

pub fn new(handle: TaskHandle) -> impl Future<Item = (), Error = Error> {
    if let Err(err) = libnotify::init("stahlnet-relay") {
        error!("Couldn't start libnotify task: {}", err);
        future::Either::A(future::ok(()))
    } else {
        info!(
            "spawned libnotify task {:?} as {}",
            handle.name(),
            handle.id()
        );

        let name = handle.name_arc();
        future::Either::B(handle.for_each(move |(_from, msg_type, data)| {
            match msg_type {
                MessageTypeID::NOTIFICATION => std::str::from_utf8(&data)
                    .and_then(|msg| {
                        let notif = Notification::new(msg, None, None);
                        notif.set_app_name(name.as_ref().map(|s| &**s));
                        notif.show().or_else(|err| {
                            error!("When sending libnotify Notification, {}", err);
                            Ok(())
                        })
                    })
                    .or_else(|err| {
                        error!("When parsing libnotify Notification message, {}", err);
                        Ok(())
                    }),
                _ => {
                    warn!("Unknown message type {}", msg_type);
                    Ok(())
                }
            }
        }))
    }
}
