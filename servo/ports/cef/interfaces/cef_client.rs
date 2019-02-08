// Copyright (c) 2015 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::mem;
use std::ptr;

//
// Implement this structure to provide handler implementations.
//
#[repr(C)]
pub struct _cef_client_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Return the handler for context menus. If no handler is provided the default
  // implementation will be used.
  //
  pub get_context_menu_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_context_menu_handler_t>,

  //
  // Return the handler for dialogs. If no handler is provided the default
  // implementation will be used.
  //
  pub get_dialog_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_dialog_handler_t>,

  //
  // Return the handler for browser display state events.
  //
  pub get_display_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_display_handler_t>,

  //
  // Return the handler for download events. If no handler is returned downloads
  // will not be allowed.
  //
  pub get_download_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_download_handler_t>,

  //
  // Return the handler for drag events.
  //
  pub get_drag_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_drag_handler_t>,

  //
  // Return the handler for find result events.
  //
  pub get_find_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_find_handler_t>,

  //
  // Return the handler for focus events.
  //
  pub get_focus_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_focus_handler_t>,

  //
  // Return the handler for geolocation permissions requests. If no handler is
  // provided geolocation access will be denied by default.
  //
  pub get_geolocation_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_geolocation_handler_t>,

  //
  // Return the handler for JavaScript dialogs. If no handler is provided the
  // default implementation will be used.
  //
  pub get_jsdialog_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_jsdialog_handler_t>,

  //
  // Return the handler for keyboard events.
  //
  pub get_keyboard_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_keyboard_handler_t>,

  //
  // Return the handler for browser life span events.
  //
  pub get_life_span_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_life_span_handler_t>,

  //
  // Return the handler for browser load status events.
  //
  pub get_load_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_load_handler_t>,

  //
  // Return the handler for off-screen rendering events.
  //
  pub get_render_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_render_handler_t>,

  //
  // Return the handler for browser request events.
  //
  pub get_request_handler: Option<extern "C" fn(
      this: *mut cef_client_t) -> *mut interfaces::cef_request_handler_t>,

  //
  // Called when a new message is received from a different process. Return true
  // (1) if the message was handled or false (0) otherwise. Do not keep a
  // reference to or attempt to access the message outside of this callback.
  //
  pub on_process_message_received: Option<extern "C" fn(this: *mut cef_client_t,
      browser: *mut interfaces::cef_browser_t,
      source_process: interfaces::cef_process_id_t,
      message: *mut interfaces::cef_process_message_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_client_t = _cef_client_t;


//
// Implement this structure to provide handler implementations.
//
pub struct CefClient {
  c_object: *mut cef_client_t,
}

impl Clone for CefClient {
  fn clone(&self) -> CefClient{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefClient {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefClient {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefClient {
  pub unsafe fn from_c_object(c_object: *mut cef_client_t) -> CefClient {
    CefClient {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_client_t) -> CefClient {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefClient {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_client_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_client_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Return the handler for context menus. If no handler is provided the default
  // implementation will be used.
  //
  pub fn get_context_menu_handler(&self) -> interfaces::CefContextMenuHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_context_menu_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for dialogs. If no handler is provided the default
  // implementation will be used.
  //
  pub fn get_dialog_handler(&self) -> interfaces::CefDialogHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_dialog_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for browser display state events.
  //
  pub fn get_display_handler(&self) -> interfaces::CefDisplayHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_display_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for download events. If no handler is returned downloads
  // will not be allowed.
  //
  pub fn get_download_handler(&self) -> interfaces::CefDownloadHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_download_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for drag events.
  //
  pub fn get_drag_handler(&self) -> interfaces::CefDragHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_drag_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for find result events.
  //
  pub fn get_find_handler(&self) -> interfaces::CefFindHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_find_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for focus events.
  //
  pub fn get_focus_handler(&self) -> interfaces::CefFocusHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_focus_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for geolocation permissions requests. If no handler is
  // provided geolocation access will be denied by default.
  //
  pub fn get_geolocation_handler(&self) -> interfaces::CefGeolocationHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_geolocation_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for JavaScript dialogs. If no handler is provided the
  // default implementation will be used.
  //
  pub fn get_jsdialog_handler(&self) -> interfaces::CefJSDialogHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_jsdialog_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for keyboard events.
  //
  pub fn get_keyboard_handler(&self) -> interfaces::CefKeyboardHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_keyboard_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for browser life span events.
  //
  pub fn get_life_span_handler(&self) -> interfaces::CefLifeSpanHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_life_span_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for browser load status events.
  //
  pub fn get_load_handler(&self) -> interfaces::CefLoadHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_load_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for off-screen rendering events.
  //
  pub fn get_render_handler(&self) -> interfaces::CefRenderHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_render_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for browser request events.
  //
  pub fn get_request_handler(&self) -> interfaces::CefRequestHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_request_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Called when a new message is received from a different process. Return true
  // (1) if the message was handled or false (0) otherwise. Do not keep a
  // reference to or attempt to access the message outside of this callback.
  //
  pub fn on_process_message_received(&self, browser: interfaces::CefBrowser,
      source_process: interfaces::CefProcessId,
      message: interfaces::CefProcessMessage) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_process_message_received.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(source_process),
          CefWrap::to_c(message)))
    }
  }
} 

impl CefWrap<*mut cef_client_t> for CefClient {
  fn to_c(rust_object: CefClient) -> *mut cef_client_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_client_t) -> CefClient {
    CefClient::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_client_t> for Option<CefClient> {
  fn to_c(rust_object: Option<CefClient>) -> *mut cef_client_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_client_t) -> Option<CefClient> {
    if c_object.is_null() {
      None
    } else {
      Some(CefClient::from_c_object_addref(c_object))
    }
  }
}
