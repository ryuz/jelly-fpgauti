use std::result::Result;
use std::error::Error;
use jelly_fpgautil as fpgautil;
//use jelly_uidmng as uidmng;

const DTS: &str = 
r#"/dts-v1/; /plugin/;

/ {
    fragment@0 {
        target = <&fpga_full>;
        overlay0: __overlay__ {
            #address-cells = <2>;
            #size-cells = <2>;
            firmware-name = "kv260_blinking_led_ps.bit.bin";
        };
    };

    fragment@1 {
        target = <&amba>;
        overlay1: __overlay__ {
            clocking0: clocking0 {
                #clock-cells = <0>;
                assigned-clock-rates = <100000000>;
                assigned-clocks = <&zynqmp_clk 71>;
                clock-output-names = "fabric_clk";
                clocks = <&zynqmp_clk 71>;
                compatible = "xlnx,fclk";
            };
        };
    };
};"#;


fn main() -> Result<(), Box<dyn Error>> {
    let dtb = fpgautil::dtc_with_str(&DTS)?;
    std::fs::write("/tmp/dtc_with_str.dtbo", dtb)?;
    Ok(())
}
