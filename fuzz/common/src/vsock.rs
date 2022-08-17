use crate::FuzzingDescriptor;
use virtio_vsock::packet::VsockPacket;

use serde::{Deserialize, Serialize};

/// All the functions that can be called on a VsockPacket
#[derive(Serialize, Deserialize, Debug)]
pub enum VsockFunctionType {
    HeaderSlice,
    Len,
    DataSlice,
    SrcCid,
    SetSrcCid { cid: u64 },
    DstCid,
    SetDstCid { cid: u64 },
    SrcPort,
    SetSrcPort { port: u32 },
    DstPort,
    SetDstPort { port: u32 },
    IsEmpty,
    SetLen { len: u32 },
    Type_,
    SetType { type_: u16 },
    Op,
    SetOp { op: u16 },
    Flags,
    SetFlags { flags: u32 },
    SetFlag { flag: u32 },
    BufAlloc,
    SetBufAlloc { buf_alloc: u32 },
    FwdCnt,
    SetFwdCnt { fwd_cnt: u32 },
    SetHeaderFromRaw { bytes: Vec<u8> },
}

impl VsockFunctionType {
    pub fn call<B: vm_memory::bitmap::BitmapSlice>(&self, packet: &mut VsockPacket<B>) {
        match self {
            VsockFunctionType::HeaderSlice => {
                packet.header_slice();
            }
            VsockFunctionType::Len => {
                packet.len();
            }
            VsockFunctionType::DataSlice => {
                packet.data_slice();
            }
            VsockFunctionType::SrcCid => {
                packet.src_cid();
            }
            VsockFunctionType::SetSrcCid { cid } => {
                packet.set_src_cid(*cid);
            }
            VsockFunctionType::DstCid => {
                packet.dst_cid();
            }
            VsockFunctionType::SetDstCid { cid } => {
                packet.set_dst_cid(*cid);
            }
            VsockFunctionType::SrcPort => {
                packet.src_port();
            }
            VsockFunctionType::SetSrcPort { port } => {
                packet.set_src_port(*port);
            }
            VsockFunctionType::DstPort => {
                packet.dst_port();
            }
            VsockFunctionType::SetDstPort { port } => {
                packet.set_dst_port(*port);
            }
            VsockFunctionType::IsEmpty => {
                packet.is_empty();
            }
            VsockFunctionType::SetLen { len } => {
                packet.set_len(*len);
            }
            VsockFunctionType::Type_ => {
                packet.type_();
            }
            VsockFunctionType::SetType { type_ } => {
                packet.set_type(*type_);
            }
            VsockFunctionType::Op => {
                packet.op();
            }
            VsockFunctionType::SetOp { op } => {
                packet.set_op(*op);
            }
            VsockFunctionType::Flags => {
                packet.flags();
            }
            VsockFunctionType::SetFlags { flags } => {
                packet.set_flags(*flags);
            }
            VsockFunctionType::SetFlag { flag } => {
                packet.set_flag(*flag);
            }
            VsockFunctionType::BufAlloc => {
                packet.buf_alloc();
            }
            VsockFunctionType::SetBufAlloc { buf_alloc } => {
                packet.set_buf_alloc(*buf_alloc);
            }
            VsockFunctionType::FwdCnt => {
                packet.fwd_cnt();
            }
            VsockFunctionType::SetFwdCnt { fwd_cnt } => {
                packet.set_fwd_cnt(*fwd_cnt);
            }
            VsockFunctionType::SetHeaderFromRaw { bytes } => {
                let _ = packet.set_header_from_raw(bytes.as_slice());
            }
        }
    }
}

// Whether we create a VsockPacket from_rx_virtq_chain or from_tx_virtq_chain
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum InitFunction {
    FromRX,
    FromTX,
}

/// Input generated by the fuzzer for fuzzing vsock_rx and vsock_tx
#[derive(Serialize, Deserialize, Debug)]
pub struct VsockInput {
    pub pkt_max_data: u32,
    pub init_function: InitFunction,
    pub descriptors: Vec<FuzzingDescriptor>,
    pub functions: Vec<VsockFunctionType>,
}
