// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//======================================================================================================================
// Imports
//======================================================================================================================

use crate::{
    runtime::{
        fail::Fail,
        types::{
            demi_qresult_t,
            demi_sgarray_t,
        },
        QDesc,
        QToken,
    },
    scheduler::SchedulerHandle,
};
use ::std::net::SocketAddrV4;

#[cfg(feature = "catcollar-libos")]
use crate::catcollar::CatcollarLibOS;
#[cfg(all(feature = "catnap-libos", target_os = "linux"))]
use crate::catnap::CatnapLibOS;
#[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
use crate::catnapw::CatnapWLibOS;
#[cfg(feature = "catnip-libos")]
use crate::catnip::CatnipLibOS;
#[cfg(feature = "catpowder-libos")]
use crate::catpowder::CatpowderLibOS;

//======================================================================================================================
// Exports
//======================================================================================================================

pub use crate::inetstack::operations::OperationResult;

//======================================================================================================================
// Structures
//======================================================================================================================

/// Network LIBOS.
pub enum NetworkLibOS {
    #[cfg(feature = "catpowder-libos")]
    Catpowder(CatpowderLibOS),
    #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
    Catnap(CatnapLibOS),
    #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
    CatnapW(CatnapWLibOS),
    #[cfg(feature = "catcollar-libos")]
    Catcollar(CatcollarLibOS),
    #[cfg(feature = "catnip-libos")]
    Catnip(CatnipLibOS),
}

//======================================================================================================================
// Associated Functions
//======================================================================================================================

/// Associated functions for network LibOSes.
impl NetworkLibOS {
    /// Creates a socket.
    pub fn socket(
        &mut self,
        domain: libc::c_int,
        socket_type: libc::c_int,
        protocol: libc::c_int,
    ) -> Result<QDesc, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.socket(domain, socket_type, protocol),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.socket(domain, socket_type, protocol),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.socket(domain, socket_type, protocol),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.socket(domain, socket_type, protocol),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.socket(domain, socket_type, protocol),
        }
    }

    /// Binds a socket to a local address.
    pub fn bind(&mut self, sockqd: QDesc, local: SocketAddrV4) -> Result<(), Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.bind(sockqd, local),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.bind(sockqd, local),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.bind(sockqd, local),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.bind(sockqd, local),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.bind(sockqd, local),
        }
    }

    /// Marks a socket as a passive one.
    pub fn listen(&mut self, sockqd: QDesc, backlog: usize) -> Result<(), Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.listen(sockqd, backlog),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.listen(sockqd, backlog),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.listen(sockqd, backlog),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.listen(sockqd, backlog),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.listen(sockqd, backlog),
        }
    }

    /// Accepts an incoming connection on a TCP socket.
    pub fn accept(&mut self, sockqd: QDesc) -> Result<QToken, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.accept(sockqd),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.accept(sockqd),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.accept(sockqd),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.accept(sockqd),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.accept(sockqd),
        }
    }

    /// Initiates a connection with a remote TCP pper.
    pub fn connect(&mut self, sockqd: QDesc, remote: SocketAddrV4) -> Result<QToken, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.connect(sockqd, remote),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.connect(sockqd, remote),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.connect(sockqd, remote),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.connect(sockqd, remote),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.connect(sockqd, remote),
        }
    }

    /// Closes a socket.
    pub fn close(&mut self, sockqd: QDesc) -> Result<(), Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.close(sockqd),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.close(sockqd),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.close(sockqd),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.close(sockqd),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.close(sockqd),
        }
    }

    /// Pushes a scatter-gather array to a TCP socket.
    pub fn push(&mut self, sockqd: QDesc, sga: &demi_sgarray_t) -> Result<QToken, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.push(sockqd, sga),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.push(sockqd, sga),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.push(sockqd, sga),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.push(sockqd, sga),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.push(sockqd, sga),
        }
    }

    /// Pushes a scatter-gather array to a UDP socket.
    pub fn pushto(&mut self, sockqd: QDesc, sga: &demi_sgarray_t, to: SocketAddrV4) -> Result<QToken, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.pushto(sockqd, sga, to),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.pushto(sockqd, sga, to),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.pushto(sockqd, sga, to),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.pushto(sockqd, sga, to),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.pushto(sockqd, sga, to),
        }
    }

    /// Pops data from a socket.
    pub fn pop(&mut self, sockqd: QDesc) -> Result<QToken, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.pop(sockqd),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.pop(sockqd),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.pop(sockqd),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.pop(sockqd),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.pop(sockqd),
        }
    }

    /// Waits for any operation in an I/O queue.
    pub fn poll(&mut self) {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.poll_bg_work(),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.poll(),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.poll(),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.poll(),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.poll_bg_work(),
        }
    }

    /// Waits for any operation in an I/O queue.
    pub fn schedule(&mut self, qt: QToken) -> Result<SchedulerHandle, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.schedule(qt),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.schedule(qt),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.schedule(qt),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.schedule(qt),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.schedule(qt),
        }
    }

    pub fn pack_result(&mut self, handle: SchedulerHandle, qt: QToken) -> Result<demi_qresult_t, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.pack_result(handle, qt),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.pack_result(handle, qt),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.pack_result(handle, qt),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.pack_result(handle, qt),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.pack_result(handle, qt),
        }
    }

    /// Allocates a scatter-gather array.
    pub fn sgaalloc(&self, size: usize) -> Result<demi_sgarray_t, Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.sgaalloc(size),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.sgaalloc(size),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.sgaalloc(size),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.sgaalloc(size),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.sgaalloc(size),
        }
    }

    /// Releases a scatter-gather array.
    pub fn sgafree(&self, sga: demi_sgarray_t) -> Result<(), Fail> {
        match self {
            #[cfg(feature = "catpowder-libos")]
            NetworkLibOS::Catpowder(libos) => libos.sgafree(sga),
            #[cfg(all(feature = "catnap-libos", target_os = "linux"))]
            NetworkLibOS::Catnap(libos) => libos.sgafree(sga),
            #[cfg(all(feature = "catnapw-libos", target_os = "windows"))]
            NetworkLibOS::CatnapW(libos) => libos.sgafree(sga),
            #[cfg(feature = "catcollar-libos")]
            NetworkLibOS::Catcollar(libos) => libos.sgafree(sga),
            #[cfg(feature = "catnip-libos")]
            NetworkLibOS::Catnip(libos) => libos.sgafree(sga),
        }
    }
}
