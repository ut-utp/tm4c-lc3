use lc3_traits::peripherals::timers::{
    TimerArr, TimerId, TimerMiscError, TimerState, TimerStateMismatch, Timers,
};

// timing errors occuring during scan cycles (input and ouput errors)
// errors handling overwriting handlers? Can timers have multiple handlers?
use lc3_isa::Word;
//use std::time::Duration;
use core::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use tm4c123x_hal::{timer, timer::*, timer::Timer, time::*};
use tm4c123x_hal::tm4c123x::{TIMER0, TIMER1};
use core::marker::PhantomData;

use tm4c123x_hal::{Peripherals, prelude::*};
use tm4c123x_hal::time::MegaHertz;
use tm4c123x_hal::sysctl::Clocks;
//use time;
//use timer;
 enum PhysicalTimers{
    T0(Timer<TIMER0>),
    T1(Timer<TIMER1>)
 }
// The term “Single Shot” signifies a single pulse output of some duration.
 pub struct TimersShim<'a> {
     states: TimerArr<TimerState>,
     times: TimerArr<Word>,
     phys_timers: Vec<PhysicalTimers>,
     flags: TimerArr<Option<&'a AtomicBool>>,
     clock_setup: [u32; 2],
     //timer_struct: [Timer; 2]
     //guards: TimerArr<Option<timer::Guard>>,
 }

 impl Default for TimersShim<'_> {
     fn default() -> Self {
            let t1 = Peripherals::take().unwrap().TIMER0;
    let time_init1 = tm4c123x_hal::timer::Timer::timer0::<MegaHertz>(
        t1,
        MegaHertz(80),
        &sys_init(),
        &Clocks{osc:Hertz(80000000), sysclk:Hertz(80000000)},
    );
    let time_init2 = tm4c123x_hal::timer::Timer::timer1::<MegaHertz>(
        Peripherals::take().unwrap().TIMER1,
        MegaHertz(80),
        &sys_init(),
        &Clocks{osc:Hertz(80000000), sysclk:Hertz(80000000)},
    );
         Self {
             states: TimerArr([TimerState::Disabled; TimerId::NUM_TIMERS]),
             times: TimerArr([0u16; TimerId::NUM_TIMERS]), // unlike gpio, interrupts occur on time - not on bit change
             flags: TimerArr([None; TimerId::NUM_TIMERS]),
//             guards: TimerArr([None, None]),

             phys_timers: vec![PhysicalTimers::T0(time_init1), PhysicalTimers::T1(time_init2)],
             clock_setup: [80000000, 80000000]
         }
     }
 }

 impl TimersShim<'_> {
     pub fn new() -> Self {
         Self::default()
     }
 }

 impl<'a> Timers<'a> for TimersShim<'a> {
     fn set_state(&mut self, timer: TimerId, state: TimerState) -> Result<(), TimerMiscError> {
//         use TimerState::*;
//         self.states[timer] = match state {
//             Repeated => {
//                 match self.guards[timer] {
//                     Some(_) => {
//                         // let g = self.guards[timer].take().unwrap();
//                         // drop(g);
//                         // // drop(x);
//                          state
//                     }
//                     None => state,
//                 }
//             }
//             SingleShot => {
//                 match self.guards[timer] {
//                     Some(_) => {
//                         // let g = self.guards[timer].take().unwrap();
//                         // drop(g);
//                         // //drop(x);
//                         state
//                     }
//                     None => state,
//                 }
//             }
//             Disabled => state,
//         };

         Ok(())
     }

    fn get_state(&self, timer: TimerId) -> TimerState {
        self.states[timer]
    }

     fn set_period(&mut self, timer: TimerId, milliseconds: Word) -> Result<(), TimerMiscError> {




//         //  use TimerState::*;

          self.times[timer] = milliseconds;
           match timer{
             T0 => {
                let curr_timer = self.phys_timers.remove(0);
                match curr_timer{
                PhysicalTimers::T0(mut v) =>{
                  let clk_freq = self.clock_setup[0];
                  let tp_millis = (1/clk_freq)*1000;
                  let divider = (milliseconds as u32)/tp_millis;
                  let ticks_new = clk_freq/divider;

                   Peripherals::take().unwrap().TIMER0.tav.write(|w| unsafe { w.bits(ticks_new) });
                   Peripherals::take().unwrap().TIMER0.tailr.write(|w| unsafe { w.bits(ticks_new) });

                    // // start counter
                    Peripherals::take().unwrap().TIMER0.ctl.modify(|_, w|
                        w.taen().set_bit()
                    );
                    self.phys_timers.insert(0,PhysicalTimers::T0(v));
                }
                _=> {}
            }
             }

             T1 => {}

           }
       //   let sys_sp = self.phys_timers[T0].clocks;
//         //  let timer_init = timer::Timer::new();

         // match self.guards[timer] {
//         //      Some(_) => {
//         //          let g = self.guards[timer].take().unwrap();
//         //          drop(g);

//         //      },
//         //      None => {}
//         //  }

//         //  match self.states[timer] {
//         //      Repeated => {
//         //          match self.flags[timer] {
//         //              Some(b) => {
//         //                  let guard = {
//         //                      timer_init.schedule_repeating(time::Duration::milliseconds(milliseconds as i64), move || {
//         //                      //self.flags[timer].unwrap().store(true, Ordering::SeqCst);
//         //                      b.store(true, Ordering::SeqCst);
//         //                      })

//         //                  };

//         //                  self.guards[timer] = Some(guard);
//         //              },
//         //              None => {
//         //                  unreachable!();
//         //              }

//         //          }
//         //      },
//         //      SingleShot => {
//         //          match self.flags[timer] {
//         //              Some(b) => {
//         //                  let guard = {
//         //                          timer_init.schedule_with_delay(time::Duration::milliseconds(milliseconds as i64), move || {
//         //                      //self.flags[timer].unwrap().store(true, Ordering::SeqCst);
//         //                          b.store(true, Ordering::SeqCst);
//         //                      })
//         //                   };

//         //                   self.guards[timer] = Some(guard);
//         //              }
//         //              None => {
//         //                  unreachable!();
//         //              }
//         //          }
//         //      },
           //    Disabled => {
            //       unreachable!();
            //   }

//         //  }

           Ok(())

//         unimplemented!()
     }

    fn get_period(&self, timer: TimerId) -> Word {
        self.times[timer]
    }

    fn register_interrupt_flag(&mut self, timer: TimerId, flag: &'a AtomicBool) {
        // self.flags[timer] = match self.flags[timer] {
        //     None => Some(flag),
        //     Some(_) => unreachable!(),
        // }
    }

    fn interrupt_occurred(&self, timer: TimerId) -> bool {
        // match self.flags[timer] {
        //     Some(flag) => {
        //         let occurred = flag.load(Ordering::SeqCst);
        //         self.interrupts_enabled(timer) && occurred
        //     }
        //     None => unreachable!(),
        // }
        false
    }

    fn reset_interrupt_flag(&mut self, timer: TimerId) {
        // match self.flags[timer] {
        //     Some(flag) => flag.store(false, Ordering::SeqCst),
        //     None => unreachable!(),
        // }
    }

    // TODO: review whether we want Interrupt state or interrupts_enabled bool state
    fn interrupts_enabled(&self, timer: TimerId) -> bool {
        match self.get_state(timer) {
            SingleShot => true,
            Repeating => true,
            Disabled => false,
        }
    }
 }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     use lc3_traits::peripherals::timers::{Timer::*, Timers};

// //     #[test]
// //     fn get_disabled() {
// //         let shim = TimersShim::new();
// //         assert_eq!(shim.get_state(T0).unwrap(), TimerState::Disabled);
// //     }

// //     #[test]
// //      fn get_singleshot() {
// //         let mut shim = TimersShim::new();
// //         let res = shim.set_state(T0, TimerState::SingleShot);
// //         assert_eq!(shim.get_state(T0).unwrap(), TimerState::SingleShot);
// //     }

// //     #[test]
// //      fn get_repeated() {
// //         let mut shim = TimersShim::new();
// //         let res = shim.set_state(T0, TimerState::Repeated);
// //         assert_eq!(shim.get_state(T0).unwrap(), TimerState::Repeated);
// //     }

// //     #[test]
// //      fn get_set_period_singleshot() {
// //         let mut shim = TimersShim::new();
// //         let res = shim.set_state(T0, TimerState::SingleShot);
// //         shim.set_period(T0, 200);
// //         assert_eq!(shim.get_period(T0).unwrap(), 200);
// //     }

// //     #[test]
// //      fn get_set_period_repeated() {
// //         let mut shim = TimersShim::new();
// //         let res = shim.set_state(T0, TimerState::Repeated);
// //         shim.set_period(T0, 200);
// //         assert_eq!(shim.get_period(T0).unwrap(), 200);
// //     }

// // }

fn sys_init() -> tm4c123x_hal::sysctl::PowerControl{
    let p_st = Peripherals::take().unwrap();
    let mut sc = p_st.SYSCTL.constrain();
    sc.power_control
   // tm4c123x_hal::sysctl::PowerControl{_0: ()}
}

fn scratch(){

    let t = Peripherals::take().unwrap().TIMER0;
    t.ctl.modify(|_, w|
                        w.taen().clear_bit()
                        .tben().clear_bit()
                        );

    let time_init = tm4c123x_hal::timer::Timer::timer0::<MegaHertz>(
        t,
        MegaHertz(80),
        &sys_init(),
        &Clocks{osc:Hertz(80000000), sysclk:Hertz(80000000)},
    );
}
