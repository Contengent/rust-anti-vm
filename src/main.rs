use std::arch::asm;
use std::process::{
    Command
};

use regex::Regex;

fn main() {
    println!("Running VM-Detections...");
    println!("-------------------------");
    println!("- Detecting hyper-v CPU VendorID... [{}]", check_vendor_id());
    println!("- Is GatewayIP webpage missing... [{}]", curl_gateway_webpage());
    println!("- Is GatewayIP response speed high... [{}]", "null")
}



fn get_gateway_ip() -> String {
    let gateway_regex = Regex::new(r"Gateway(\s+(\.\s+)+): (?P<gatewayIP>.*)")
        .unwrap();

    let ipconfig_output = Command::new("ipconfig")
        .output()
        .expect(".");

    let raw_ipconfig_output = String::from_utf8(ipconfig_output.stdout)
        .expect(".");

    let Some(gatewayStr) = gateway_regex.captures(&*raw_ipconfig_output) else {
        return "Error: No gateway found. Or maybe you're on linux.".to_string();
    };

    return (&gatewayStr["gatewayIP"]).to_string();
}

fn curl_gateway_webpage() -> bool {
    let curl_output = Command::new("curl")
        .arg(get_gateway_ip())
        .output()
        .expect(".");

    let raw_ipconfig_output = String::from_utf8(curl_output.stdout)
        .expect(".");


    if(raw_ipconfig_output == ""){
        return true;
    } else {
        return false;
    }


}

fn check_gateway_response() -> bool {
    // todo
    return false;
}

fn get_vendor_id() -> String {
    // Set output buffer
    let mut name_buf = [0_u8; 12];

    // Start running ASM
    unsafe {
        asm!(
            // save 'rbx' in non-volatile state (?)
            "push rbx",

            // move 'get VendorID' function to 'eax'
            "mov eax, 0x40000000",

            // call cpuid function
            "cpuid",

            // Move return 'string' to ebx, ecx, and edx.
            "mov [rdi], ebx",
            "mov [rdi + 4], ecx",
            "mov [rdi + 8], edx",

            // move saved 'rbx' values back into palce
            "pop rbx",

            // Pass inputs, and Get outputs
            in("rdi") name_buf.as_mut_ptr(),
            inout("eax") 0 => _,
            out("ecx") _,
            out("edx") _,
        );
    }

    // Dereference nameBuffer*, and encode in UTF8
    let vendor_id = core::str::from_utf8(&name_buf).unwrap().to_string();

    // return CPU's Vendor ID
    return vendor_id;
}
fn check_vendor_id() -> bool{
    if get_vendor_id() == "Microsoft Hv"{
        return true;
    } else{
        return false;
    }
}