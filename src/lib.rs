pub use egl;
pub use libloading;

use std::os::raw::c_void;


pub struct Egl14 {
    instance: egl::Instance<egl::Dynamic<libloading::Library,egl::EGL1_4>>,
    ctx: Option<EglCtx>,
}

pub struct EglCtx{
            display: egl::Display,
            raw_egl_ctx: egl::Context,
            config: egl::Config,
            surface: Option<egl::Surface>,
}

impl Egl14{
    pub fn entry_load()->Result<Self,String>{
        return Ok(Self{
            instance:unsafe{
                egl::DynamicInstance::<egl::EGL1_4>::load_required().map_err(|e|format!("{:?}",e))?
            },
            ctx:None,
        })
    }

    pub fn entry_init(&mut self)->Result<(),String>{
        let egl_ins = &self.instance;
        let display = egl_ins
            .get_display(egl::DEFAULT_DISPLAY)
            .ok_or_else(|| {
                let msg = "❌ noget default display";
                msg
            })?;


        egl_ins
            .initialize(display)
            .map_err(|e| {
                let err_msg = format!("❌ Failed to init display {:?}", e);
                err_msg
            })?;

        println!("✅ Inited display");

        println!("⌛ Choose config which matched the attributes we wanted");
        let attributes: Vec<egl::Int> = [
            egl::RED_SIZE,
            8,
            egl::GREEN_SIZE,
            8,
            egl::BLUE_SIZE,
            8,
            egl::NONE,
        ]
        .to_vec();
        let config: egl::Config = egl_ins
            .choose_first_config(display, &attributes)
            .map_err(|e| {
                let err_msg = format!("❌ Failed to choose first config {:?}", e);
                err_msg
            })?
            .ok_or_else(|| {
                let msg = "❌ No matched config ";
                println!("{:?}", msg);
                msg
            })?;

        println!("✅ Config choosed");
        // >>

        let context_attributes = [
            egl::CONTEXT_MAJOR_VERSION,
            2,
            egl::CONTEXT_MINOR_VERSION,
            0,
            egl::NONE,
        ];

        // << create_context
        println!("⌛ Creating context");
        let ctx = egl_ins
            .create_context(display, config, None, Some(&context_attributes))
            .map_err(|e| {
                let err_msg = format!("❌ Failed to create context {:?}", e);
                err_msg
            })?;
        println!("✅ Created context");

        self.ctx= Some(EglCtx{
            display: display,
            raw_egl_ctx:ctx,
            config: config,
            surface: None,
        });

        Ok(())
    }
    
    pub fn alone_create_surface(&self,platform_window:*mut c_void)->Result<egl::Surface,String>{
        let egl_ctx_helper = self.ctx.as_ref().unwrap();
        let egl_ins = &self.instance;
        let surface = unsafe {
           egl_ins 
                .create_window_surface(
                    egl_ctx_helper.display,
                    egl_ctx_helper.config,
                    platform_window as egl::NativeWindowType,
                    None,
                )
                .map_err(|e| {
                    let err_msg = format!("❌ Failed to create window surface {:?}", e);
                    err_msg
                })?
        };
        println!("✅  Created window surface");
        return Ok(surface);
    }

    pub fn attach_surface_to_ctx(&mut self,surface: egl::Surface)->Result<(),String>{
        println!("⌛ Attach an EGL rendering context to EGL surfaces");
        self.instance
            .make_current(self.ctx.as_mut().unwrap().display, Some(surface), Some(surface), Some(self.ctx.as_mut().unwrap().raw_egl_ctx))
            .map_err(|e| {
                let err_msg =format!("❌ Failed to attach egl rendering context to egl surfaces {:?}",e);
                err_msg
            })?;
        println!("✅ Attached an EGL rendering context to EGL surfaces");
        self.ctx.as_mut().unwrap().surface = Some(surface);
        return Ok(());
    }


}
