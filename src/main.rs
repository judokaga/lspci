use std::collections::HashMap;
use std::fs::File;
use std::fmt;
use std::io::{BufReader, BufRead};

type VendorID = i32;
type DeviceID = i32;

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:04x}:{:04x}]", self.vendor, self.device)
    }
}

#[derive(Eq, PartialEq, Hash)]
struct ID {
    vendor: i32,
    device: i32,
}

fn read_line(reader: &mut BufReader<File>) -> String {
    loop {
        let mut buf = String::new();
        let n = reader.read_line(&mut buf).unwrap();
        if n == 0 {
            return buf;
        }
        // not empty line or comment
        if !(buf.trim().is_empty() || buf.trim().starts_with('#')) {
            return buf;
        }
    }
}

fn read_vendors(mut m: HashMap<VendorID, Vendor>, line: String, reader: &mut BufReader<File>) -> HashMap<VendorID, Vendor> {
    if line.is_empty() {
        return m;
    }
    // ignore device classes, subclasses and programming interfaces
    // and everything that follows
    if line.starts_with("C ") {
        return m;
    }
    let mut it = line.splitn(2, "  ");
    let vendor_id = VendorID::from_str_radix(it.next().unwrap(), 16).unwrap();
    let vendor_name = it.next().unwrap().to_owned().trim_matches('\n').to_owned();

    let mut m1 = HashMap::new();
    loop {
        let mut buf = read_line(reader);

        // TODO: subvendor/subdevice
        // ignored for now
        while buf.starts_with("\t\t") {
            buf = read_line(reader);
        }
    
        if !buf.starts_with('\t') {
            let vendor = Vendor {
                id: vendor_id,
                name: vendor_name,
                device_ids: m1,
            };
            m.insert(vendor.id, vendor);
            return read_vendors(m, buf, reader);
        }
        buf.remove(0);
        let mut it = buf.splitn(2, "  ");
        let device_id = DeviceID::from_str_radix(it.next().unwrap(), 16).unwrap();
        let device_str = it.next().unwrap();
        m1.insert(device_id, device_str.to_owned());
    }
}

struct Vendor {
    id: VendorID,
    name: String,
    device_ids: HashMap<DeviceID, String>,
}

fn read_ids(fname: &'static str) -> HashMap<VendorID, Vendor> {
    let f = File::open(fname).unwrap();
    let mut reader = BufReader::new(f);
    let m = HashMap::new();

    let line = read_line(&mut reader);
    return read_vendors(m, line, &mut reader);
}

fn id_to_str(id: &ID) -> String {
    let m = read_ids("./pci.ids");
    match m.get(&id.vendor) {
        Some(vendor) => {
            let mut vendor_name = vendor.name.to_owned();
            let device_str = vendor.device_ids.get(&id.device).unwrap_or(&"Unknown PIC ID".to_owned()).to_owned();
            vendor_name += " ";
            vendor_name += &device_str;
            vendor_name
        },
        None => "Unknown PIC ID".to_owned(),
    }
}

fn main() {
    let i1 = ID {
        vendor: 0x0014,
        device: 0x7a02,
    };
    let i2 = ID {
        vendor: 33,
        device: 33,
    };
    let i3 = ID {
        vendor: 0x001c,
        device: 0x0001,
    };
    println!("{}: {}", &i1, id_to_str(&i1));
    println!("{}: {}", &i2, id_to_str(&i2));
    println!("{}: {}", &i3, id_to_str(&i3));
}
