> [!NOTE]  
> This is based on jon-zu's fork of unknowntrojan's egui-d3d9: https://github.com/jon-zu/egui-d3d9

# egui-d3d9

<a href="https://crates.io/crates/egui-d3d9"><img src="https://img.shields.io/crates/v/egui-d3d9.svg"></img></a>

D3D9 backend for [egui](https://github.com/emilk/egui).
Primarily intended for source games like CS:GO and GMod.

It's not perfect by far, but it'll do. This is a rewrite of a fork I had of sy1ntexx's [egui-d3d11](https://github.com/sy1ntexx/egui-d3d11). The input manager and the example code are still mostly from that repository. A lot of the general structure was inherited, as I found it quite intuitive.

Turns out, porting from D3D11 down to D3D9 is harder than *just* doing D3D9.