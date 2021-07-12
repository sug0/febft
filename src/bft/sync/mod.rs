//! Implements the synchronization phase from the Mod-SMaRt protocol.
//!
//! This code allows a replica to change its view, where a new
//! leader is elected.

use std::marker::PhantomData;

use crate::bft::log::Log;
use crate::bft::core::server::ViewInfo;
use crate::bft::communication::Node;
use crate::bft::communication::message::{
    Header,
};
use crate::bft::executable::{
    Service,
    Request,
    Reply,
    State,
};

pub type TimeoutSeqNo = i32;

enum ProtoPhase {
    // nothing is happening, there are no client
    // requests to be ordered
    Init,
    // we are watching the timers of a batch
    // of client requests (not the same batch
    // used during consensus)
    WatchingTimeout,
    // first batch of client requests has timed
    // out, and we reset its timers
    Watching2ndTimeout,
    // we are running the stopping phase of the
    // Mod-SMaRt protocol
    Stopping(usize),
    // we are still running the stopping phase of
    // Mod-SMaRt, but we have either locally triggered
    // a view change, or received at least f+1 STOP msgs,
    // so we don't need to broadcast a new STOP msgs
    Stopping2(usize),
    // we are running the STOP-DATA phase of Mod-SMaRt
    StoppingData(usize),
    // we are running the SYNC phase of Mod-SMaRt
    Syncing(usize),
}

// TODO: finish statuses returned from `process_message`
pub enum SynchronizerStatus {
    /// We are not running the view change protocol.
    Nil,
    /// The view change protocol is currently running.
    Running,
    /// We installed a new view, resulted from running the
    /// view change protocol.
    NewView(ViewInfo),
}

// TODO:
// - the fields in this struct
// - TboQueue for sync phase messages?
pub struct Synchronizer<S: Service> {
    phase: ProtoPhase,
    // TODO: probably remove this...
    timeout_seq: TimeoutSeqNo,
    view: ViewInfo,
    _phantom: PhantomData<S>,
}

impl<S> Synchronizer<S>
where
    S: Service + Send + 'static,
    State<S>: Send + 'static,
    Request<S>: Send + 'static,
    Reply<S>: Send + 'static,
{
    pub fn new(view: ViewInfo) -> Self {
        Self {
            view,
            _phantom: PhantomData,
            phase: ProtoPhase::Init,
            timeout_seq: 0,
        }
    }

    /// Install a new view received from the CST protocol, or from
    /// running the view change protocol.
    pub fn install_view(&mut self, view: ViewInfo) {
        // FIXME: is the following line necessary?
        //self.phase = ProtoPhase::Init;
        self.view = view;
    }

    /// Advances the state of the view change state machine.
    pub fn process_message(
        &mut self,
        header: Header,
        message: () /*ViewChangeMessage*/,
        log: &mut Log<State<S>, Request<S>, Reply<S>>,
        node: &mut Node<S::Data>,
    ) -> SynchronizerStatus {
        unimplemented!()
    }

    /// Handle a timeout received from the timeouts layer.
    pub fn timed_out(&mut self) -> SynchronizerStatus {
        // TODO:
        // - on the first timeout we forward pending requests to
        //   the leader
        // - on the second timeout, we start a view change by
        //   broadcasting a STOP message
        unimplemented!()
    }

    /// Returns some information regarding the current view, such as
    /// the number of faulty replicas the system can tolerate.
    pub fn view(&self) -> &ViewInfo {
        &self.view
    }
}
