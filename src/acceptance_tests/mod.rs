use self::fixture::run_acceptance_test;

mod fixture;

#[test]
fn add_sp_e_timing() { run_acceptance_test("acceptance/add_sp_e_timing") }

#[test]
fn call_cc_timing() { run_acceptance_test("acceptance/call_cc_timing") }

#[test]
fn call_cc_timing2() { run_acceptance_test("acceptance/call_cc_timing2") }

#[test]
fn call_timing() { run_acceptance_test("acceptance/call_timing") }

#[test]
fn call_timing2() { run_acceptance_test("acceptance/call_timing2") }

#[test]
fn di_timing() { run_acceptance_test("acceptance/di_timing") }

#[test]
fn div_timing() { run_acceptance_test("acceptance/div_timing") }

#[test]
fn ei_timing() { run_acceptance_test("acceptance/ei_timing") }

#[test]
fn halt_ime1() { run_acceptance_test("acceptance/halt_ime1") }

#[test]
fn if_ie_registers() { run_acceptance_test("acceptance/if_ie_registers") }

#[test]
fn intr_timing() { run_acceptance_test("acceptance/intr_timing") }

#[test]
fn jp_cc_timing() { run_acceptance_test("acceptance/jp_cc_timing") }

#[test]
fn jp_timing() { run_acceptance_test("acceptance/jp_timing") }

#[test]
fn ld_hl_sp_e_timing() { run_acceptance_test("acceptance/ld_hl_sp_e_timing") }

#[test]
fn oam_bits() { run_acceptance_test("acceptance/oam_bits") }

#[test]
fn oam_dma_restart() { run_acceptance_test("acceptance/oam_dma_restart") }

#[test]
fn oam_dma_timing() { run_acceptance_test("acceptance/oam_dma_timing") }

#[test]
fn pop_timing() { run_acceptance_test("acceptance/pop_timing") }

#[test]
fn push_timing() { run_acceptance_test("acceptance/push_timing") }

#[test]
fn rapid_di_ei() { run_acceptance_test("acceptance/rapid_di_ei") }

#[test]
fn ret_timing() { run_acceptance_test("acceptance/ret_timing") }

#[test]
fn reti_timing() { run_acceptance_test("acceptance/reti_timing") }

#[test]
fn ret_cc_timing() { run_acceptance_test("acceptance/ret_cc_timing") }

#[test]
fn reti_intr_timing() { run_acceptance_test("acceptance/reti_intr_timing") }

#[test]
fn rst_timing() { run_acceptance_test("acceptance/rst_timing") }

#[test]
fn mbc1_rom_4banks() { run_acceptance_test("emulator-only/mbc1_rom_4banks") }
