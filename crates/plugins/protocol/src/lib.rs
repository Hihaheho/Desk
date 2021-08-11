#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
use native::*;

use bevy::prelude::*;
use core::ProtocolSystem;
use futures::prelude::*;
use protocol::{ClientName, Clients, Command, Event, Login};
use std::borrow::Cow;
use tracing::error;

const DEFAULT_CLIENT: ClientName =
    ClientName(Cow::Borrowed("desk-plugin-protocol: default client"));

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        let app = app
            .init_resource::<Clients>()
            .add_event::<Command>()
            .add_event::<Event>()
            .add_system(receive_events.system().label(ProtocolSystem::ReceiveEvents))
            .add_system(
                handle_events
                    .system()
                    .label(ProtocolSystem::HandleEvents)
                    .after(ProtocolSystem::ReceiveEvents)
                    .before(ProtocolSystem::SendCommands),
            )
            .add_system(send_commands.system().label(ProtocolSystem::SendCommands));
        app.add_startup_system(connect_websocket.system());
        #[cfg(target_arch = "wasm32")]
        app.add_system(set_client.system());
    }
}

fn handle_events() {}

fn send_commands(_commands: EventReader<Command>, mut clients: ResMut<Clients>) {
    if let Some(client) = clients.get_mut(&DEFAULT_CLIENT) {
        // for command in commands.iter() {
        //     let command = command.clone();
        let command = Command::Login(Login {
            token: vec![1, 2, 3].into(),
        });
        let mut sender = client.sender();
        block_on(async move {
            sender.send(command).await.unwrap_or_else(|err| {
                error!("{}", err);
            })
        });
        // }
    }
}
fn receive_events(mut events: EventWriter<Event>, mut clients: ResMut<Clients>) {
    if let Some(client) = clients.get_mut(&DEFAULT_CLIENT) {
        if let Some(vec) = client.poll_once() {
            for event in vec {
                events.send(event);
            }
        }
    }
}
