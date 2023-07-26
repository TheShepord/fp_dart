use std::sync::RwLock;

use fastpair::bluetooth::{Adapter, BleAdapter, BleDataTypeId, BleDevice, BluetoothError, Device};
use flutter_rust_bridge::StreamSink;
use futures::executor;

static NAME_STREAM: RwLock<Option<StreamSink<String>>> = RwLock::new(None);

pub fn init() -> String {
    let run = async {
        println!("start making adapter");

        let mut adapter = BleAdapter::default().await?;
        adapter.start_scan()?;

        let datatype_selector = vec![BleDataTypeId::ServiceData16BitUuid];
        while let advertisement = adapter
            .next_advertisement(Some(&datatype_selector))
            .await
            .unwrap()
        {
            for service_data in advertisement.service_data_16bit_uuid()? {
                let uuid = service_data.uuid();

                // This is a Fast Pair device.
                if uuid == 0x2cfe {
                    let addr = advertisement.address();
                    let ble_device = BleDevice::new(addr).await?;
                    let name = ble_device.name()?;
                    println!("device: {}", name);

                    match NAME_STREAM.try_read() {
                        Ok(s) => match s.as_ref() {
                            Some(s) => {
                                s.add(name);
                            }
                            None => println!("stream is none"),
                        },
                        Err(_) => println!("error reading aaa"),
                    }
                }
            }
        }
        Err(BluetoothError::Internal(String::from("rip")))
    };

    executor::block_on(run).unwrap()
}

pub fn event_stream(s: StreamSink<String>) -> Result<(), anyhow::Error> {
    let mut stream = NAME_STREAM.write().unwrap();
    *stream = Some(s);
    Ok(())
}
