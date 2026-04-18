use std::{
    ffi::{CString, c_char},
    sync::atomic::{AtomicBool, Ordering},
};

pub static RUNNING: AtomicBool = AtomicBool::new(true);

extern "C" fn handle_sigint(_sig: i32) {
    RUNNING.store(false, Ordering::SeqCst);
}

unsafe extern "C" {
    fn signal(sig: i32, handler: extern "C" fn(i32)) -> usize;
}

pub fn setup_signal_handler() {
    unsafe {
        signal(2, handle_sigint);
    }
}

unsafe extern "C" {
    pub fn server_event_team_created(
        team_uuid: *const c_char,
        team_name: *const c_char,
        user_uuid: *const c_char,
    ) -> i32;
    pub fn server_event_user_loaded(user_uuid: *const c_char, user_name: *const c_char) -> i32;
    pub fn server_event_user_created(user_uuid: *const c_char, user_name: *const c_char) -> i32;
    pub fn server_event_user_logged_in(user_uuid: *const c_char) -> i32;
    pub fn server_event_user_logged_out(user_uuid: *const c_char) -> i32;
    pub fn server_event_private_message_sended(
        s_uuid: *const c_char,
        r_uuid: *const c_char,
        c_body: *const c_char,
    ) -> i32;
    pub fn server_event_user_subscribed(team_uuid: *const c_char, user_uuid: *const c_char) -> i32;
    pub fn server_event_user_unsubscribed(
        team_uuid: *const c_char,
        user_uuid: *const c_char,
    ) -> i32;
    pub fn server_event_channel_created(
        team_uuid: *const c_char,
        channel_uuid: *const c_char,
        name: *const c_char,
    ) -> i32;
    pub fn server_event_thread_created(
        team_uuid: *const c_char,
        channel_uuid: *const c_char,
        thead_uuid: *const c_char,
        title: *const c_char,
        body: *const c_char,
    ) -> i32;
    pub fn server_event_reply_created(
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        reply_body: *const c_char,
    ) -> i32;
}

pub fn call_user_loaded(uuid: &str, name: &str) {
    if let (Ok(c_uuid), Ok(c_name)) = (CString::new(uuid), CString::new(name)) {
        unsafe { server_event_user_loaded(c_uuid.as_ptr(), c_name.as_ptr()) };
    }
}

pub fn call_user_logged_in(uuid: &str) {
    if let Ok(c_uuid) = CString::new(uuid) {
        unsafe { server_event_user_logged_in(c_uuid.as_ptr()) };
    }
}

pub fn call_user_created(uuid: &str, name: &str) {
    if let (Ok(c_uuid), Ok(c_name)) = (CString::new(uuid), CString::new(name)) {
        unsafe { crate::ffi::server_event_user_created(c_uuid.as_ptr(), c_name.as_ptr()) };
    }
}

pub fn call_user_logged_out(uuid: &str) {
    if let Ok(c_uuid) = CString::new(uuid) {
        unsafe { crate::ffi::server_event_user_logged_out(c_uuid.as_ptr()) };
    }
}

pub fn call_private_message_sended(sender_uuid: &str, receiver_uuid: &str, body: &str) {
    if let (Ok(s_uuid), Ok(r_uuid), Ok(c_body)) = (
        CString::new(sender_uuid),
        CString::new(receiver_uuid),
        CString::new(body),
    ) {
        unsafe {
            crate::ffi::server_event_private_message_sended(
                s_uuid.as_ptr(),
                r_uuid.as_ptr(),
                c_body.as_ptr(),
            )
        };
    }
}

pub fn call_user_subscribed(team_uuid: &str, user_uuid: &str) {
    if let (Ok(t), Ok(u)) = (CString::new(team_uuid), CString::new(user_uuid)) {
        unsafe { server_event_user_subscribed(t.as_ptr(), u.as_ptr()) };
    }
}

pub fn call_user_unsubscribed(team_uuid: &str, user_uuid: &str) {
    if let (Ok(t), Ok(u)) = (CString::new(team_uuid), CString::new(user_uuid)) {
        unsafe { server_event_user_unsubscribed(t.as_ptr(), u.as_ptr()) };
    }
}

pub fn call_team_created(team_uuid: &str, name: &str, user_uuid: &str) {
    if let (Ok(t), Ok(n), Ok(u)) = (
        CString::new(team_uuid),
        CString::new(name),
        CString::new(user_uuid),
    ) {
        unsafe { server_event_team_created(t.as_ptr(), n.as_ptr(), u.as_ptr()) };
    }
}

pub fn call_channel_created(team_uuid: &str, channel_uuid: &str, name: &str) {
    if let (Ok(t), Ok(c), Ok(n)) = (
        CString::new(team_uuid),
        CString::new(channel_uuid),
        CString::new(name),
    ) {
        unsafe { server_event_channel_created(t.as_ptr(), c.as_ptr(), n.as_ptr()) };
    }
}

pub fn call_thread_created(
    channel_uuid: &str,
    thread_uuid: &str,
    user_uuid: &str,
    title: &str,
    body: &str,
) {
    if let (Ok(c), Ok(t), Ok(u), Ok(ti), Ok(b)) = (
        CString::new(channel_uuid),
        CString::new(thread_uuid),
        CString::new(user_uuid),
        CString::new(title),
        CString::new(body),
    ) {
        unsafe {
            server_event_thread_created(c.as_ptr(), t.as_ptr(), u.as_ptr(), ti.as_ptr(), b.as_ptr())
        };
    }
}

pub fn call_reply_created(thread_uuid: &str, user_uuid: &str, reply_body: &str) {
    if let (Ok(t), Ok(u), Ok(r)) = (
        CString::new(thread_uuid),
        CString::new(user_uuid),
        CString::new(reply_body),
    ) {
        unsafe { server_event_reply_created(t.as_ptr(), u.as_ptr(), r.as_ptr()) };
    }
}
