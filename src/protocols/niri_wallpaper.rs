use std::collections::HashMap;

use smithay::reexports::wayland_server::{
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource, Weak,
};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;

use super::raw::niri_wallpaper::v1::server::niri_wallpaper_manager_v1::{
    self, NiriWallpaperManagerV1,
};
use super::raw::niri_wallpaper::v1::server::niri_wallpaper_surface_v1::{
    self, NiriWallpaperSurfaceV1,
};

pub struct NiriWallpaperManagerState {
    /// Maps each tagged surface to its declared workspace index (IPC-style, 1-based).
    pub surface_workspace: HashMap<Weak<WlSurface>, u32>,
}

pub struct NiriWallpaperManagerGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

pub trait NiriWallpaperHandler {
    fn niri_wallpaper_manager_state(&mut self) -> &mut NiriWallpaperManagerState;
}

impl NiriWallpaperManagerState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<NiriWallpaperManagerV1, NiriWallpaperManagerGlobalData>,
        D: Dispatch<NiriWallpaperManagerV1, ()>,
        D: Dispatch<NiriWallpaperSurfaceV1, Weak<WlSurface>>,
        D: NiriWallpaperHandler,
        D: 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let global_data = NiriWallpaperManagerGlobalData {
            filter: Box::new(filter),
        };
        display.create_global::<D, NiriWallpaperManagerV1, _>(1, global_data);
        Self {
            surface_workspace: HashMap::new(),
        }
    }

    /// Returns the workspace index for a surface, if one was declared.
    pub fn workspace_for(&self, surface: &WlSurface) -> Option<u32> {
        self.surface_workspace.get(&surface.downgrade()).copied()
    }
}

impl<D> GlobalDispatch<NiriWallpaperManagerV1, NiriWallpaperManagerGlobalData, D>
    for NiriWallpaperManagerState
where
    D: GlobalDispatch<NiriWallpaperManagerV1, NiriWallpaperManagerGlobalData>,
    D: Dispatch<NiriWallpaperManagerV1, ()>,
    D: Dispatch<NiriWallpaperSurfaceV1, Weak<WlSurface>>,
    D: NiriWallpaperHandler,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _handle: &DisplayHandle,
        _client: &Client,
        manager: New<NiriWallpaperManagerV1>,
        _manager_state: &NiriWallpaperManagerGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(manager, ());
    }

    fn can_view(client: Client, global_data: &NiriWallpaperManagerGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<NiriWallpaperManagerV1, (), D> for NiriWallpaperManagerState
where
    D: Dispatch<NiriWallpaperManagerV1, ()>,
    D: Dispatch<NiriWallpaperSurfaceV1, Weak<WlSurface>>,
    D: NiriWallpaperHandler,
    D: 'static,
{
    fn request(
        state: &mut D,
        _client: &Client,
        _manager: &NiriWallpaperManagerV1,
        request: niri_wallpaper_manager_v1::Request,
        _data: &(),
        _dh: &DisplayHandle,
        init: &mut DataInit<'_, D>,
    ) {
        match request {
            niri_wallpaper_manager_v1::Request::GetWallpaperSurface {
                id,
                surface,
                workspace_index,
            } => {
                let weak = surface.downgrade();
                state
                    .niri_wallpaper_manager_state()
                    .surface_workspace
                    .insert(weak.clone(), workspace_index);
                init.init(id, weak);
            }
            niri_wallpaper_manager_v1::Request::Destroy => {}
        }
    }
}

impl<D> Dispatch<NiriWallpaperSurfaceV1, Weak<WlSurface>, D> for NiriWallpaperManagerState
where
    D: Dispatch<NiriWallpaperSurfaceV1, Weak<WlSurface>>,
    D: NiriWallpaperHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _resource: &NiriWallpaperSurfaceV1,
        request: niri_wallpaper_surface_v1::Request,
        _data: &Weak<WlSurface>,
        _dh: &DisplayHandle,
        _init: &mut DataInit<'_, D>,
    ) {
        match request {
            niri_wallpaper_surface_v1::Request::Destroy => {}
        }
    }

    fn destroyed(
        state: &mut D,
        _client: smithay::reexports::wayland_server::backend::ClientId,
        _resource: &NiriWallpaperSurfaceV1,
        data: &Weak<WlSurface>,
    ) {
        state
            .niri_wallpaper_manager_state()
            .surface_workspace
            .remove(data);
    }
}

#[macro_export]
macro_rules! delegate_niri_wallpaper {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::niri_wallpaper::v1::server::niri_wallpaper_manager_v1::NiriWallpaperManagerV1: $crate::protocols::niri_wallpaper::NiriWallpaperManagerGlobalData
        ] => $crate::protocols::niri_wallpaper::NiriWallpaperManagerState);

        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::niri_wallpaper::v1::server::niri_wallpaper_manager_v1::NiriWallpaperManagerV1: ()
        ] => $crate::protocols::niri_wallpaper::NiriWallpaperManagerState);

        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::niri_wallpaper::v1::server::niri_wallpaper_surface_v1::NiriWallpaperSurfaceV1: smithay::reexports::wayland_server::Weak<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface>
        ] => $crate::protocols::niri_wallpaper::NiriWallpaperManagerState);
    };
}
