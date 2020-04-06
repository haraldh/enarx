// SPDX-License-Identifier: Apache-2.0

use nolibc::x86_64::error::Number as ErrNo;
use nolibc::x86_64::syscall::Number as SysCall;
use sgx_types::ssa::StateSaveArea;

use crate::handler::{Context, Handler, Print};
use crate::Layout;

#[no_mangle]
pub extern "C" fn event(
    _rdi: u64,
    _rsi: u64,
    _rdx: u64,
    layout: &Layout,
    _r8: u64,
    _r9: u64,
    aex: &mut StateSaveArea,
    ctx: &Context,
) {
    let mut h = Handler::new(layout, aex, ctx);

    match unsafe { core::slice::from_raw_parts(h.aex.gpr.rip as *const u8, 2) } {
        // syscall
        [0x0f, 0x05] => {
            aex.gpr.rax = match h.aex.gpr.rax.into() {
                SysCall::READ => h.read(),
                SysCall::READV => h.readv(),
                SysCall::WRITE => h.write(),
                SysCall::WRITEV => h.writev(),
                SysCall::EXIT => h.exit(None),
                SysCall::GETUID => h.getuid(),
                SysCall::ARCH_PRCTL => h.arch_prctl(),
                SysCall::EXIT_GROUP => h.exit_group(None),
                SysCall::SET_TID_ADDRESS => h.set_tid_address(),
                SysCall::BRK => h.brk(),

                syscall => {
                    h.print("unsupported syscall: ");
                    h.print(&syscall);
                    h.print("\n");
                    ErrNo::ENOSYS.into_syscall()
                }
            };

            aex.gpr.rip += 2;
        }

        r => {
            let opcode = (r[0] as u16) << 8 | r[1] as u16;
            h.print("unsupported opcode: ");
            h.print(&opcode);
            h.print("\n");
            h.exit(1)
        }
    };
}