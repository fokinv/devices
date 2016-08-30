/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_adapter::BluetoothAdapter as BluetoothAdapterBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_adapter::Adapter as BluetoothAdapterAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
use empty::BluetoothAdapter as BluetoothAdapterEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_adapter::FakeBluetoothAdapter;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_device::BluetoothDevice as BluetoothDeviceBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_device::Device as BluetoothDeviceAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
use empty::BluetoothDevice as BluetoothDeviceEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_device::FakeBluetoothDevice;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_characteristic::Characteristic as BluetoothGATTCharacteristicAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
use empty::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_characteristic::FakeBluetoothGATTCharacteristic;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_descriptor::BluetoothGATTDescriptor as BluetoothGATTDescriptorBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_descriptor::Descriptor as BluetoothGATTDescriptorAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
use empty::BluetoothGATTDescriptor as BluetoothGATTDescriptorEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_descriptor::FakeBluetoothGATTDescriptor;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_service::BluetoothGATTService as BluetoothGATTServiceBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_service::Service as BluetoothGATTServiceAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
use empty::BluetoothGATTService as BluetoothGATTServiceEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_service::FakeBluetoothGATTService;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as BluetoothDiscoverySessionBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_discovery_session::DiscoverySession as BluetoothDiscoverySessionAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
use empty::BluetoothDiscoverySession as BluetoothDiscoverySessionEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_discovery_session::FakeBluetoothDiscoverySession;

use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct BluetoothAdapter {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    adapter: Arc<BluetoothAdapterBluez>,
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    adapter: Arc<BluetoothAdapterAndroid>,
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    adapter: Arc<BluetoothAdapterEmpty>,
    #[cfg(feature = "bluetooth-test")]
    adapter: Arc<FakeBluetoothAdapter>,
}

#[derive(Debug)]
pub struct BluetoothDiscoverySession {
    adapter: RefCell<BluetoothAdapter>,
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    session: Arc<BluetoothDiscoverySessionBluez>,
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    session: Arc<BluetoothDiscoverySessionAndroid>,
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    session: Arc<BluetoothDiscoverySessionEmpty>,
    #[cfg(feature = "bluetooth-test")]
    session: Arc<FakeBluetoothDiscoverySession>,
}

#[derive(Clone, Debug)]
pub struct BluetoothDevice {
    adapter: RefCell<BluetoothAdapter>,
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    device: Arc<BluetoothDeviceBluez>,
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    device: Arc<BluetoothDeviceAndroid>,
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    device: Arc<BluetoothDeviceEmpty>,
    #[cfg(feature = "bluetooth-test")]
    device: Arc<FakeBluetoothDevice>,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTService {
    device: RefCell<BluetoothDevice>,
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    gatt_service: Arc<BluetoothGATTServiceBluez>,
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    gatt_service: Arc<BluetoothGATTServiceAndroid>,
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    gatt_service: Arc<BluetoothGATTServiceEmpty>,
    #[cfg(feature = "bluetooth-test")]
    gatt_service: Arc<FakeBluetoothGATTService>,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTCharacteristic {
    service: RefCell<BluetoothGATTService>,
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    gatt_characteristic: Arc<BluetoothGATTCharacteristicBluez>,
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    gatt_characteristic: Arc<BluetoothGATTCharacteristicAndroid>,
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    gatt_characteristic: Arc<BluetoothGATTCharacteristicEmpty>,
    #[cfg(feature = "bluetooth-test")]
    gatt_characteristic: Arc<FakeBluetoothGATTCharacteristic>,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTDescriptor {
    characteristic: RefCell<BluetoothGATTCharacteristic>,
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    gatt_descriptor: Arc<BluetoothGATTDescriptorBluez>,
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    gatt_descriptor: Arc<BluetoothGATTDescriptorAndroid>,
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    gatt_descriptor: Arc<BluetoothGATTDescriptorEmpty>,
    #[cfg(feature = "bluetooth-test")]
    gatt_descriptor: Arc<FakeBluetoothGATTDescriptor>,
}

impl BluetoothAdapter {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(BluetoothAdapter {adapter: Arc::new(bluez_adapter)})
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let blurdroid_adapter = try!(BluetoothAdapterAndroid::get_adapter());
        Ok(BluetoothAdapter {adapter: Arc::new(blurdroid_adapter)})
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let adapter = try!(BluetoothAdapterEmpty::init());
        Ok(BluetoothAdapter {adapter: Arc::new(adapter)})
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        Ok(BluetoothAdapter{adapter: FakeBluetoothAdapter::new_empty()})
    }

    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    fn get_adapter(&self) -> Arc<BluetoothAdapterBluez> {
        self.adapter.clone()
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    fn get_adapter(&self) -> Arc<BluetoothAdapterAndroid> {
        self.adapter.clone()
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    fn get_adapter(&self) -> Arc<BluetoothAdapterEmpty> {
        self.adapter.clone()
    }

    #[cfg(feature = "bluetooth-test")]
    fn get_adapter(&self) -> Arc<FakeBluetoothAdapter> {
        self.adapter.clone()
    }

    pub fn get_id(&self) -> String {
        self.get_adapter().get_id()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_id(&self, id: String) {
        self.get_adapter().set_id(id)
    }

    pub fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>> {
        let device_list = try!(self.get_adapter().get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(self.clone(), device)).collect())
    }

    pub fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            if try!(device.get_address()) == address {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        self.get_adapter().get_address()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        self.get_adapter().set_address(address)
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        self.get_adapter().get_name()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_name(&self, name: String) -> Result<(), Box<Error>> {
        self.get_adapter().set_name(name)
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        self.get_adapter().get_alias()
    }

    pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        self.get_adapter().set_alias(value)
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_class()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_class(&self, class: u32) -> Result<(), Box<Error>> {
        self.get_adapter().set_class(class)
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_powered()
    }

    pub fn set_powered(&self, value: bool) -> Result<(), Box<Error>> {
        self.get_adapter().set_powered(value)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn is_present(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_present()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_present(&self, value: bool) -> Result<(), Box<Error>> {
        self.get_adapter().set_present(value)
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_discoverable()
    }

    pub fn set_discoverable(&self, value: bool) -> Result<(), Box<Error>> {
        self.get_adapter().set_discoverable(value)
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_pairable()
    }

    pub fn set_pairable(&self, value: bool) -> Result<(), Box<Error>> {
        self.get_adapter().set_pairable(value)
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_pairable_timeout()
    }

    pub fn set_pairable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        self.get_adapter().set_pairable_timeout(value)
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_discoverable_timeout()
    }

    pub fn set_discoverable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        self.get_adapter().set_discoverable_timeout(value)
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_discovering()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_discovering(&self, is_discovering: bool) -> Result<(), Box<Error>> {
        self.get_adapter().set_discovering(is_discovering)
    }

    pub fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        BluetoothDiscoverySession::create_session(self.clone())
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.get_adapter().get_uuids()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        self.get_adapter().set_uuids(uuids)
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        self.get_adapter().get_vendor_id_source()
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_vendor_id()
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_product_id()
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_device_id()
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        self.get_adapter().get_modalias()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>> {
        self.get_adapter().set_modalias(modalias)
    }
}

impl BluetoothDiscoverySession {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn create_session(adapter: BluetoothAdapter) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let bluez_session = try!(BluetoothDiscoverySessionBluez::create_session(adapter.get_id()));
        Ok(BluetoothDiscoverySession{
            adapter: RefCell::new(adapter),
            session: Arc::new(bluez_session),
        })
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn create_session(adapter: BluetoothAdapter) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let blurdroid_session = try!(BluetoothDiscoverySessionAndroid::create_session(adapter.get_adapter()));
        Ok(BluetoothDiscoverySession{
            adapter: RefCell::new(adapter),
            session: Arc::new(blurdroid_session),
        })
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    pub fn create_session(adapter: BluetoothAdapter) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let empty_session = try!(BluetoothDiscoverySessionEmpty::create_session(adapter.get_adapter()));
        Ok(BluetoothDiscoverySession{
            adapter: RefCell::new(adapter),
            session: Arc::new(empty_session),
        })
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_session(adapter: BluetoothAdapter) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let test_session = try!(FakeBluetoothDiscoverySession::create_session(adapter.get_adapter()));
        Ok(BluetoothDiscoverySession{
            adapter: RefCell::new(adapter),
            session: Arc::new(test_session),
        })
    }

    pub fn get_adapter(&self) -> BluetoothAdapter {
        self.adapter.borrow_mut().clone()
    }

    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        self.session.start_discovery()
    }

    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        self.session.stop_discovery()
    }
}

impl BluetoothDevice {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn create_device(adapter: BluetoothAdapter, device: String) -> BluetoothDevice {
        BluetoothDevice{
            adapter: RefCell::new(adapter),
            device: Arc::new(BluetoothDeviceBluez::new(device.clone())),
        }
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn create_device(adapter: BluetoothAdapter, device: String) -> BluetoothDevice {
        BluetoothDevice{
            adapter: RefCell::new(adapter.clone()),
            device: Arc::new(BluetoothDeviceAndroid::new(adapter.get_adapter(), device.clone())),
        }
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    pub fn create_device(adapter: BluetoothAdapter, device: String) -> BluetoothDevice {
        BluetoothDevice{
            adapter: RefCell::new(adapter),
            device: Arc::new(BluetoothDeviceEmpty::new(device.clone())),
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_device(adapter: BluetoothAdapter, device_id: String) -> BluetoothDevice {
        BluetoothDevice {
            adapter: RefCell::new(adapter.clone()),
            device: FakeBluetoothDevice::new_empty(adapter.get_adapter(), device_id),
        }
    }

    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    fn get_device(&self) -> Arc<BluetoothDeviceBluez> {
        self.device.clone()
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    fn get_device(&self) -> Arc<BluetoothDeviceAndroid> {
        self.device.clone()
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    fn get_device(&self) -> Arc<BluetoothDeviceEmpty> {
        self.device.clone()
    }

    #[cfg(feature = "bluetooth-test")]
    fn get_device(&self) -> Arc<FakeBluetoothDevice> {
        self.device.clone()
    }

    pub fn get_id(&self) -> String {
        self.get_device().get_id()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_id(&self, id: String) {
        self.get_device().set_id(id)
    }

    pub fn get_adapter(&self) -> BluetoothAdapter {
        self.adapter.borrow_mut().clone()
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        self.get_device().get_address()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        self.get_device().set_address(address)
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        self.get_device().get_name()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_name(&self, address: String) -> Result<(), Box<Error>> {
        self.get_device().set_name(address)
    }

    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        self.get_device().get_icon()
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        self.get_device().get_class()
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        self.get_device().get_appearance()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_appearance(&self, appearance: u16) -> Result<(), Box<Error>> {
        self.get_device().set_appearance(appearance)
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.get_device().get_uuids()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        self.get_device().set_uuids(uuids)
    }

    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_paired()
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_connected()
    }

    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_trusted()
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_blocked()
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        self.get_device().get_alias()
    }

    pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        self.get_device().set_alias(value)
    }

    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_legacy_pairing()
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        self.get_device().get_vendor_id_source()
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        self.get_device().get_vendor_id()
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        self.get_device().get_product_id()
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        self.get_device().get_device_id()
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        self.get_device().get_modalias()
    }

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        self.get_device().get_rssi()
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        self.get_device().get_tx_power()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_tx_power(&self, tx_power: i16) -> Result<(), Box<Error>> {
        self.get_device().set_tx_power(tx_power)
    }

    pub fn get_gatt_services(&self) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        let services = try!(self.get_device().get_gatt_services());
        Ok(services.into_iter().map(|service| BluetoothGATTService::create_service(self.clone(), service)).collect())
    }

    pub fn connect(&self) -> Result<(), Box<Error>> {
        self.get_device().connect()
    }

    pub fn disconnect(&self) -> Result<(), Box<Error>> {
        self.get_device().disconnect()
    }

    pub fn connect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        self.get_device().connect_profile(uuid)
    }

    pub fn disconnect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        self.get_device().disconnect_profile(uuid)
    }

    pub fn pair(&self) -> Result<(), Box<Error>> {
        self.get_device().pair()
    }

    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        self.get_device().cancel_pairing()
    }
}

impl BluetoothGATTService {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn create_service(device: BluetoothDevice, service: String) -> BluetoothGATTService {
        BluetoothGATTService{
            device: RefCell::new(device),
            gatt_service: Arc::new(BluetoothGATTServiceBluez::new(service.clone())),
        }
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn create_service(device: BluetoothDevice, service: String) -> BluetoothGATTService {
        BluetoothGATTService{
            device: RefCell::new(device.clone()),
            gatt_service: Arc::new(BluetoothGATTServiceAndroid::new(device.get_device(), service.clone())),
        }
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    pub fn create_service(device: BluetoothDevice, service: String) -> BluetoothGATTService {
        BluetoothGATTService{
            device: RefCell::new(device),
            gatt_service: Arc::new(BluetoothGATTServiceEmpty::new(service.clone())),
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_service(device: BluetoothDevice, service: Arc<FakeBluetoothGATTService>) -> BluetoothGATTService {
        BluetoothGATTService{
            device: RefCell::new(device),
            gatt_service: service,
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn new_mock(device: BluetoothDevice, service_id: String) -> BluetoothGATTService {
        BluetoothGATTService{
            device: RefCell::new(device.clone()),
            gatt_service: FakeBluetoothGATTService::new_empty(device.get_device(), service_id),
        }
    }

    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    fn get_gatt_service(&self) -> Arc<BluetoothGATTServiceBluez> {
        self.gatt_service.clone()
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    fn get_gatt_service(&self) -> Arc<BluetoothGATTServiceAndroid> {
        self.gatt_service.clone()
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    fn get_gatt_service(&self) -> Arc<BluetoothGATTServiceEmpty> {
        self.gatt_service.clone()
    }

    #[cfg(feature = "bluetooth-test")]
    fn get_gatt_service(&self) -> Arc<FakeBluetoothGATTService> {
        self.gatt_service.clone()
    }

    pub fn get_id(&self) -> String {
        self.get_gatt_service().get_id()
    }

    pub fn get_device(&self) -> BluetoothDevice {
        self.device.borrow_mut().clone()
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        self.get_gatt_service().get_uuid()
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_uuid(&self, uuid: String) -> Result<(), Box<Error>> {
        self.get_gatt_service().set_uuid(uuid)
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        self.get_gatt_service().is_primary()
    }

    pub fn get_includes(&self) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        let services = try!(self.get_gatt_service().get_includes());
        Ok(services.into_iter().map(|service| BluetoothGATTService::create_service(self.get_device(), service)).collect())
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<BluetoothGATTCharacteristic>, Box<Error>> {
        let characteristics = try!(self.get_gatt_service().get_gatt_characteristics());
        Ok(characteristics.into_iter().map(|characteristic| BluetoothGATTCharacteristic::create_characteristic(self.clone(), characteristic)).collect())
    }
}

impl BluetoothGATTCharacteristic {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn create_characteristic(service: BluetoothGATTService, characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic{
            service: RefCell::new(service),
            gatt_characteristic: Arc::new(BluetoothGATTCharacteristicBluez::new(characteristic.clone()))
        }
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn create_characteristic(service: BluetoothGATTService, characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic{
            service: RefCell::new(service.clone()),
            gatt_characteristic: Arc::new(BluetoothGATTCharacteristicAndroid::new(service.get_gatt_service(), characteristic.clone()))
        }
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    pub fn create_characteristic(service: BluetoothGATTService, characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic{
            service: RefCell::new(service),
            gatt_characteristic: Arc::new(BluetoothGATTCharacteristicEmpty::new(characteristic.clone()))
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_characteristic(service: BluetoothGATTService, characteristic: Arc<FakeBluetoothGATTCharacteristic>) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic{
            service: RefCell::new(service),
            gatt_characteristic: characteristic
        }
    }

    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    fn get_gatt_characteristic(&self) -> Arc<BluetoothGATTCharacteristicBluez> {
        self.gatt_characteristic.clone()
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    fn get_gatt_characteristic(&self) -> Arc<BluetoothGATTCharacteristicAndroid> {
        self.gatt_characteristic.clone()
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    fn get_gatt_characteristic(&self) -> Arc<BluetoothGATTCharacteristicEmpty> {
        self.gatt_characteristic.clone()
    }

    #[cfg(feature = "bluetooth-test")]
    fn get_gatt_characteristic(&self) -> Arc<FakeBluetoothGATTCharacteristic> {
        self.gatt_characteristic.clone()
    }

    pub fn get_id(&self) -> String {
        self.get_gatt_characteristic().get_id()
    }

    pub fn get_service(&self) -> BluetoothGATTService {
        self.service.borrow_mut().clone()
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        self.get_gatt_characteristic().get_uuid()
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_gatt_characteristic().get_value()
    }

    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        self.get_gatt_characteristic().is_notifying()
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        self.get_gatt_characteristic().get_flags()
    }

    pub fn get_gatt_descriptors(&self) -> Result<Vec<BluetoothGATTDescriptor>, Box<Error>> {
        let descriptors =  try!(self.get_gatt_characteristic().get_gatt_descriptors());
        Ok(descriptors.into_iter().map(|descriptor| BluetoothGATTDescriptor::create_descriptor(self.clone(), descriptor)).collect())
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_gatt_characteristic().read_value()
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        self.get_gatt_characteristic().write_value(values)
    }

    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        self.get_gatt_characteristic().start_notify()
    }

    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        self.get_gatt_characteristic().stop_notify()
    }
}

impl BluetoothGATTDescriptor {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn create_descriptor(characteristic: BluetoothGATTCharacteristic, descriptor: String) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor{
            characteristic: RefCell::new(characteristic),
            gatt_descriptor: Arc::new(BluetoothGATTDescriptorBluez::new(descriptor.clone()))
        }
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn create_descriptor(characteristic: BluetoothGATTCharacteristic, descriptor: String) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor{
            characteristic: RefCell::new(characteristic.clone()),
            gatt_descriptor: Arc::new(BluetoothGATTDescriptorAndroid::new(characteristic.get_gatt_characteristic(), descriptor.clone()))
        }
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    pub fn create_descriptor(characteristic: BluetoothGATTCharacteristic, descriptor: String) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor{
            characteristic: RefCell::new(characteristic),
            gatt_descriptor: Arc::new(BluetoothGATTDescriptorEmpty::new(descriptor.clone()))
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_descriptor(characteristic: BluetoothGATTCharacteristic, descriptor: Arc<FakeBluetoothGATTDescriptor>) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor{
            characteristic: RefCell::new(characteristic),
            gatt_descriptor: descriptor
        }
    }

    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    fn get_gatt_descriptor(&self) -> Arc<BluetoothGATTDescriptorBluez> {
        self.gatt_descriptor.clone()
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    fn get_gatt_descriptor(&self) -> Arc<BluetoothGATTDescriptorAndroid> {
        self.gatt_descriptor.clone()
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"), feature = "bluetooth-test")))]
    fn get_gatt_descriptor(&self) -> Arc<BluetoothGATTDescriptorEmpty> {
        self.gatt_descriptor.clone()
    }

    #[cfg(feature = "bluetooth-test")]
    fn get_gatt_descriptor(&self) -> Arc<FakeBluetoothGATTDescriptor> {
        self.gatt_descriptor.clone()
    }

    pub fn get_id(&self) -> String {
        self.get_gatt_descriptor().get_id()
    }

    pub fn get_characteristic(&self) -> BluetoothGATTCharacteristic {
        self.characteristic.borrow_mut().clone()
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        self.get_gatt_descriptor().get_uuid()
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_gatt_descriptor().get_value()
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        self.get_gatt_descriptor().get_flags()
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_gatt_descriptor().read_value()
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        self.get_gatt_descriptor().write_value(values)
    }
}
