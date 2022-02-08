use windows::{
    core::*,
    Win32::Graphics::Direct3D::*,
    Win32::Graphics::Dxgi::Common::*,
    Win32::Graphics::{
        Direct3D11::{D3D11CreateDevice, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION},
        Dxgi::*,
    },
};

fn main() {
    unsafe {
        let flags = D3D11_CREATE_DEVICE_FLAG(0);
        let feature_levels: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL_11_0;
        let num_feature_levels = 1;
        let mut d3d11_device = None;
        let mut d3d11_immediate_context = None;
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            flags,
            &feature_levels,
            num_feature_levels,
            D3D11_SDK_VERSION,
            &mut d3d11_device,
            std::ptr::null_mut(),
            &mut d3d11_immediate_context,
        )
        .unwrap();
        let d3d11_device = d3d11_device.unwrap();
        let dxgi_device = d3d11_device.cast::<IDXGIDevice>().unwrap();
        let dxgi_adapter = dxgi_device.GetAdapter().unwrap();
        let desc = dxgi_adapter.GetDesc().unwrap();
        println!("Adapter: {:?}", String::from_utf16(&desc.Description.split(|x| *x == 0).next().unwrap()).unwrap());
        let output = dxgi_adapter.EnumOutputs(0).unwrap();
        let output3 = output.cast::<IDXGIOutput3>().unwrap();

        println!("{}", output3.SupportsOverlays().as_bool());
        let flags = output3
            .CheckOverlaySupport(DXGI_FORMAT_NV12, &d3d11_device)
            .unwrap();
        println!("NV12: {}", flags_to_str(flags));
        let flags = output3
            .CheckOverlaySupport(DXGI_FORMAT_YUY2, &d3d11_device)
            .unwrap();

        fn flags_to_str(flags: u32) -> String {
            let flags = DXGI_OVERLAY_SUPPORT_FLAG(flags as i32);
            let mut result = Vec::new();
            if flags.0 & DXGI_OVERLAY_SUPPORT_FLAG_DIRECT.0 != 0 {
                result.push("DIRECT");
            }
            if flags.0 & DXGI_OVERLAY_SUPPORT_FLAG_SCALING.0 != 0 {
                result.push("SCALING");
            }
            result.join(" | ")
        }
        println!("YUY2: {}", flags_to_str(flags));
        let flags = output3
            .CheckOverlaySupport(DXGI_FORMAT_B8G8R8A8_UNORM, &d3d11_device)
            .unwrap();
        println!("B8G8R8A8_UNORM: {}", flags_to_str(flags));
        let flags = output3
            .CheckOverlaySupport(DXGI_FORMAT_R10G10B10A2_UNORM, &d3d11_device)
            .unwrap();
        println!("R10G10B10A2_UNORM: {}", flags_to_str(flags));
    }
}
