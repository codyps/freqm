use std::ffi::OsStr;
use std::time::Duration;
use serialport::prelude::*;
use super::sparse_mem::SparseMem;

// Serial settings: 9600 8N1
//
// Programming software using a 500 ms timeout (via the windows apis). Unclear if it actually does
// timeout after 500 ms or if there is a application layer wait.
//
// Send these first to enter a particular communication mode:
//
// "FALTORY": request factory mode, sent by the "SetO" factory program
// "PROGRAM": request program mode, sent by the programming software
//
//
pub fn download<P: AsRef<OsStr>>(port_name: P) -> serialport::Result<SparseMem> {
    let s = SerialPortSettings {
        baud_rate: 9600,
        flow_control: FlowControl::None,
        data_bits: DataBits::Eight,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(500),
    };

    let mut p = serialport::open_with_settings(port_name.as_ref(), &s)?;

    let sparse_mem = SparseMem::default();

    p.write_all(b"PROGRAM")?;

    {
        let mut b = [0u8; 3];
        let c = p.read(&mut b)?;
        assert_eq!(c, b.len());

        if [b'Q', b'X', 0x06] != b {
            // bad, bail out
        }
    }

    // -> 02
    // <- 49 54 45 52 4d 4e 38 52 04 56 31 30 30 f4 00 06   ITERMN8R.V100ô.. 

    // ->    52 00 40 10                                       R.@.             
    // <-    57 00 40 10 00 00 00 00 00 00 00 00 00 00 00 00   W.@............. 
    // <-    00 00 00 00 50 06                                 ....P.           
    
    // ->    52 00 10 10                                       R...             
    // <-    57 00 10 10 11 04 54 45 52 4d 4e 38 52 04 00 00   W.....TERMN8R... 
    // <-    00 f4 00 00 3d 06                                 .ô..=.           

    // ->    52 00 20 10                                       R. .             
    // <-    57 00 20 10 ff ff ff ff ff ff ff ff ff ff ff ff   W. .ÿÿÿÿÿÿÿÿÿÿÿÿ 
    // <-    ff ff ff ff 20 06                                 ÿÿÿÿ .           

    // ->    52 00 30 10                                       R.0.             
    // <-    57 00 30 10 32 30 31 35 2d 32 2d 31 31 00 00 00   W.0.2015-2-11... 
    // <-    00 00 00 00 f6 06                                 ....ö.           

    // until

    // ->    52 43 70 10                                       RCp.             
    // <-    57 43 70 10 00 00 00 4d 55 52 53 20 35 30 00 00   WCp....MURS 50.. 
    // <-    00 00 00 00 8f 06                                 .....           

    // ->    45 4e 44                                          END              
    // <-    06                                                .                

    // done

    Ok(sparse_mem)
}
