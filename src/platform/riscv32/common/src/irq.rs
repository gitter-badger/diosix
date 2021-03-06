/* RISC-V 32-bit common exception/interrupt hardware-specific code
 *
 * (c) Chris Williams, 2018.
 *
 * See LICENSE for usage and copying.
 */

/* dispatch
   Handle incoming IRQs: software exceptions and hardware interrupts
   for the high-level kernel.
   => context = context from the low-level code that picked up the IRQ
   <= return high-level description of the IRQ for the portable kernel
*/
pub fn dispatch(context: ::IRQContext) -> ::IRQ
{
  /* convert RISC-V cause codes into generic codes for the kernel.
     the top bit of the cause code is set for interrupts and clear for execeptions */
  let cause_type = match context.cause >> 31
  {
    0 => ::IRQType::Exception,
    _ => ::IRQType::Interrupt
  };
  let cause_mask = (1 << 31) - 1;
  let (fatal, cause) = match (cause_type, context.cause & cause_mask)
  {
    /* exceptions - some are labeled fatal */
    (::IRQType::Exception,  0) => (true,  ::IRQCause::InstructionAlignment),
    (::IRQType::Exception,  1) => (true,  ::IRQCause::InstructionAccess),
    (::IRQType::Exception,  2) => (true,  ::IRQCause::IllegalInstruction),
    (::IRQType::Exception,  3) => (false, ::IRQCause::Breakpoint),
    (::IRQType::Exception,  4) => (true,  ::IRQCause::LoadAlignment),
    (::IRQType::Exception,  5) => (true,  ::IRQCause::LoadAccess),
    (::IRQType::Exception,  6) => (true,  ::IRQCause::StoreAlignment),
    (::IRQType::Exception,  7) => (true,  ::IRQCause::StoreAccess),
    (::IRQType::Exception,  8) => (false, ::IRQCause::UserEnvironmentCall),
    (::IRQType::Exception,  9) => (false, ::IRQCause::SupervisorEnvironmentCall),
    (::IRQType::Exception, 11) => (false, ::IRQCause::KernelEnvironmentCall),
    (::IRQType::Exception, 12) => (false, ::IRQCause::InstructionPageFault),
    (::IRQType::Exception, 13) => (false, ::IRQCause::LoadPageFault),
    (::IRQType::Exception, 15) => (false, ::IRQCause::StorePageFault),

    /* interrupts - none are fatal */
    (::IRQType::Interrupt,  0) => (false, ::IRQCause::UserSWI),
    (::IRQType::Interrupt,  1) => (false, ::IRQCause::SupervisorSWI),
    (::IRQType::Interrupt,  3) => (false, ::IRQCause::KernelSWI),
    (::IRQType::Interrupt,  4) => (false, ::IRQCause::UserTimer),
    (::IRQType::Interrupt,  5) => (false, ::IRQCause::SupervisorTimer),
    (::IRQType::Interrupt,  7) => (false, ::IRQCause::KernelTimer),
    (::IRQType::Interrupt,  8) => (false, ::IRQCause::UserInterrupt),
    (::IRQType::Interrupt,  9) => (false, ::IRQCause::SupervisorInterrupt),
    (::IRQType::Interrupt, 11) => (false, ::IRQCause::KernelInterrupt),
    (_,  _) => (false, ::IRQCause::Unknown)
  };

  /* return structure describing this exception to the high-level kernel */
  ::IRQ
  {
    fatal: fatal,
    irq_type: cause_type,
    cause: cause,
    privilege_mode: ::PrivilegeMode::Kernel,
    pc: context.epc
  }
}
