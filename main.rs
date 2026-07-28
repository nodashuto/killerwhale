Microsoft Windows [Version 10.0.26200.8875]
(c) Microsoft Corporation. All rights reserved.

c:\Users\Molda\projects\killerwhale\src>cd ..
cd ..

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
error[E0425]: cannot find type `Grounded` in this scope
   --> src\main.rs:286:67
    |
 42 | struct isGrounded;
    | ------------------ similarly named struct `isGrounded` defined here
...
286 |     mut query: Query<(Entity, &mut Velocity), (With<Player>, With<Grounded>)>,
    |                                                                   ^^^^^^^^
    |
help: a struct with a similar name exists
    |
286 |     mut query: Query<(Entity, &mut Velocity), (With<Player>, With<isGrounded>)>,
    |                                                                   ++
help: you might be missing a type parameter
    |
283 | fn player_jump<Grounded>(
    |               ++++++++++

error[E0425]: cannot find type `Grounded` in this scope
   --> src\main.rs:294:42
    |
 42 | struct isGrounded;
    | ------------------ similarly named struct `isGrounded` defined here
...
294 |         commands.entity(entity).remove::<Grounded>();
    |                                          ^^^^^^^^
    |
help: a struct with a similar name exists
    |
294 |         commands.entity(entity).remove::<isGrounded>();
    |                                          ++
help: you might be missing a type parameter
    |
283 | fn player_jump<Grounded>(
    |               ++++++++++

error[E0425]: cannot find value `Grounded` in this scope
   --> src\main.rs:323:40
    |
 42 | struct isGrounded;
    | ------------------ similarly named unit struct `isGrounded` defined here
...
323 |         commands.entity(entity).insert(Grounded);
    |                                        ^^^^^^^^
    |
help: a unit struct with a similar name exists
    |
323 |         commands.entity(entity).insert(isGrounded);
    |                                        ++

warning: type `isGrounded` should have an upper camel case name
  --> src\main.rs:42:8
   |
42 | struct isGrounded;
   |        ^^^^^^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `IsGrounded`
   |
   = note: `#[warn(non_camel_case_types)]` (part of `#[warn(nonstandard_style)]`) on by default

For more information about this error, try `rustc --explain E0425`.
warning: `killerwhale` (bin "killerwhale") generated 1 warning
error: could not compile `killerwhale` (bin "killerwhale") due to 3 previous errors; 1 warning emitted

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 4.85s
     Running `target\debug\killerwhale.exe`
2026-07-28T12:56:16.113400Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T12:56:16.368512Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T12:56:16.368564Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T12:56:16.369297Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T12:56:53.519395Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T12:56:53.519431Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.55s
     Running `target\debug\killerwhale.exe`
2026-07-28T12:59:49.926742Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T12:59:50.179547Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T12:59:50.179594Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T12:59:50.180232Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:00:22.317980Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:00:22.318012Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.92s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:21:21.620843Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:21:21.863358Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:21:21.863410Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:21:21.864262Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:21:41.752072Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:21:41.752111Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.59s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:27:15.838564Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:27:16.082455Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:27:16.082501Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:27:16.083060Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:27:36.468458Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:27:36.468493Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.54s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:33:44.038159Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:33:44.279327Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:33:44.279376Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:33:44.279954Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:33:56.868396Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:33:56.868429Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.61s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:36:27.871902Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:36:28.116124Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:36:28.116171Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:36:28.116769Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:37:13.701625Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:37:13.701660Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.61s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:37:48.797157Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:37:49.038743Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:37:49.038795Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:37:49.039360Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:38:33.966228Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:38:33.966263Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.61s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:38:56.847412Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:38:57.101030Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:38:57.101079Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:38:57.101707Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:39:17.333456Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:39:17.333490Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.60s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:41:06.144103Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:41:06.404168Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:41:06.404225Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:41:06.404872Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:41:11.633413Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:41:11.633448Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 1.79s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:42:06.090400Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:42:06.349595Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:42:06.349647Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:42:06.350344Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:42:24.849889Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:42:24.849922Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.67s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:43:58.990351Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:43:59.230829Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:43:59.230879Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:43:59.231455Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:44:22.600029Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:44:22.600066Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.62s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:45:12.361031Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:45:12.594505Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:45:12.594555Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:45:12.595178Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:45:58.916612Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:45:58.916646Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.61s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:49:42.448347Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:49:42.706524Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:49:42.706576Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:49:42.707287Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:49:47.398942Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:49:47.398978Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.57s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:50:58.067755Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:50:58.326354Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:50:58.326453Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:50:58.327389Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:51:03.099797Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:51:03.099835Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.60s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:51:29.765819Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:51:30.026088Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:51:30.026137Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:51:30.026870Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:51:33.132156Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:51:33.132195Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.60s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:53:02.527810Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:53:02.764766Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:53:02.764811Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:53:02.765361Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:53:07.866699Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:53:07.866733Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.54s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:53:33.095771Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:53:33.350712Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:53:33.350763Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:53:33.351511Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:53:37.165695Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:53:37.165732Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.63s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:54:02.165916Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:54:02.404753Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:54:02.404830Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:54:02.405433Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:54:11.416482Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:54:11.416516Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.58s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:54:21.192021Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:54:21.432972Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:54:21.433016Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:54:21.433561Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:54:24.748975Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:54:24.749009Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.56s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:54:43.463659Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:54:43.700298Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:54:43.700354Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:54:43.700972Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:54:56.233143Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:54:56.233176Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.57s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:55:13.466301Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:55:13.701708Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:55:13.701754Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:55:13.702315Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:55:17.598650Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:55:17.598688Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.57s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:55:28.343815Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:55:28.590265Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:55:28.590310Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:55:28.590873Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:55:30.814794Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:55:30.814829Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>intensity: 4000.0,
intensity: 4000.0,
'intensity:' は、内部コマンドまたは外部コマンド、
操作可能なプログラムまたはバッチ ファイルとして認識されていません。

c:\Users\Molda\projects\killerwhale>cargo run --features bevy/dynamic_linking
cargo run --features bevy/dynamic_linking
   Compiling killerwhale v0.1.0 (C:\Users\Molda\projects\killerwhale)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.65s
     Running `target\debug\killerwhale.exe`
2026-07-28T13:55:58.792945Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060", vendor: 4318, device: 9351, device_type: DiscreteGpu, device_pci_bus_id: "0000:01:00.0", driver: "NVIDIA", driver_info: "591.74", backend: Vulkan, subgroup_min_size: 32, subgroup_max_size: 32, transient_saves_memory: false }
2026-07-28T13:55:59.031824Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-07-28T13:55:59.031870Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-07-28T13:55:59.032425Z  INFO bevy_winit::system: Creating new window killerwhale (65v0)
2026-07-28T13:56:36.000003Z  INFO bevy_window::system: No windows are open, exiting
2026-07-28T13:56:36.000040Z  INFO bevy_winit::system: Closing window 65v0

c:\Users\Molda\projects\killerwhale>