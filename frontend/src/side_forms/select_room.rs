use std::str::FromStr;

use crate::{libs::send_message, structs::Notification};
use futures::channel::mpsc::UnboundedSender;
use gloo_net::websocket::Message;
use macros::string;
use structs::{MessageDTO, Player, PlayerRole};
use sycamore::prelude::*;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::{MouseEvent, SubmitEvent};

#[component(inline_props)]
pub fn SelectRoom(
    send: Signal<Option<UnboundedSender<Message>>>,
    notification: Signal<Notification>,
) -> View {
    let action = create_signal(Action::CreateRoom);
    let user_name = create_signal(string!());
    let room_code = create_signal(string!());
    // create_effect(move || {
    //     console_log!("user: name: {}",user_name.get_clone());
    // });
    view! {
        button(class=match action.get(){
            Action::CreateRoom => "selected",
            _ => "",
        },on:click=move|ev:MouseEvent|{
            ev.prevent_default();
            if action.get() == Action::JoinRoom {
                action.set(Action::CreateRoom);
            }
        }){"Create Room"}
        button(class=match action.get(){
            Action::JoinRoom => "selected",
            _ => "",
        },on:click=move|ev:MouseEvent|{
            ev.prevent_default();
            if action.get() == Action::CreateRoom {
                action.set(Action::JoinRoom);
            }
        }){"Join Room"}
        form(on:submit=move|ev: SubmitEvent|{
            ev.prevent_default();
            spawn_local(async move {
                if let Err(e) = match action.get(){
                    Action::CreateRoom => {
                        let size = user_name.with(|n| n.len());
                        if size == 0 {
                            console_log!("User name is empty - Select Room");
                            Err(string!("Debe ingresar el nombre"))
                        } else {
                            let player = Player::new(user_name.get_clone(), PlayerRole::Master );
                            console_log!("Logging in as {:#?}",player.role());
                            send_message(*send, MessageDTO::AddPlayer { player }).await
                        }
                    },
                    Action::JoinRoom => {
                        match Uuid::from_str(room_code.get_clone().as_str()) {
                            Ok(room_id) => {
                                let size = user_name.with(|n| n.len());
                                if size == 0 {
                                    Err(string!("Debe ingresar el nombre"))
                                } else {
                                    let player = Player::new(user_name.get_clone(), PlayerRole::Player { room: room_id });
                                    send_message(*send, MessageDTO::AddPlayer { player }).await
                                }
                            },
                            Err(e) => Err(format!("Codigo de sala inválido: {e}")),
                        }

                    },
                } {
                    notification.set(Notification::Error(format!("Error: {e}")));
                }
            });
            // users.update(|users| {
            //     users.insert(new_user.id(), new_user);
            // });
        }){
            (match action.get(){
                Action::CreateRoom => view!{
                    input(placeholder="Nombre", bind:value=user_name){}
                    input(r#type="submit", value="Crear sala"){}
                },
                Action::JoinRoom => view!{
                    input(placeholder="Nombre", bind:value=user_name){}
                    input(placeholder="Código de sala", bind:value=room_code){}
                    input(r#type="submit", value="Unirse"){}
                },
            })
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Action {
    CreateRoom,
    JoinRoom,
}
