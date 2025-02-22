use crate::hcore::{fs,
                   package::{Identifiable,
                             PackageIdent}};
use handlebars::{Context,
                 Handlebars,
                 Helper,
                 HelperDef,
                 HelperResult,
                 Output,
                 RenderContext,
                 RenderErrorReason};
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct PkgPathForHelper;

impl HelperDef for PkgPathForHelper {
    fn call<'reg: 'rc, 'rc>(&self,
                            h: &Helper<'rc>,
                            _r: &'reg Handlebars<'reg>,
                            ctx: &'rc Context,
                            _rc: &mut RenderContext<'reg, 'rc>,
                            out: &mut dyn Output)
                            -> HelperResult {
        let param =
            h.param(0)
             .and_then(|v| v.value().as_str())
             .and_then(|v| PackageIdent::from_str(v).ok())
             .ok_or_else(|| {
                 RenderErrorReason::Other("Invalid package identifier for \"pkgPathFor\"".to_string())
             })?;
        let deps =
            serde_json::from_value::<Vec<PackageIdent>>(ctx.data()["pkg"]["deps"].clone()).unwrap();
        let target_pkg =
            deps.iter()
                .find_map(|ident| {
                    if ident.satisfies(&param) {
                        Some(fs::pkg_install_path(ident, Some(&*fs::FS_ROOT_PATH)).to_string_lossy()
                                                                               .into_owned())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
        out.write(target_pkg.as_ref())?;
        Ok(())
    }
}

pub static PKG_PATH_FOR: PkgPathForHelper = PkgPathForHelper;
