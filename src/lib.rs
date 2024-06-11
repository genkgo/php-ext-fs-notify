use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, EventKind, event::*};
use phper::{functions::Argument, modules::Module, php_get_module, values::ZVal, classes::{ClassEntity, ClassEntry, Visibility}};
use phper::arrays::{ZArray, InsertKey, IterKey};
use phper::objects::{StateObj};
use std::convert::Infallible;
use std::path::PathBuf;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        "fs-notify",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    let mut event = ClassEntity::new("FsNotify\\Event");
    event.add_property("kind", Visibility::Private, ());
    event.add_property("paths", Visibility::Private, ());
    event.add_method("__construct", Visibility::Private, |_t, _a| Ok::<_, Infallible>(()));
    event.add_method("getPaths", Visibility::Public, |this: &mut StateObj<()>, _: &mut [ZVal]| {
        let paths = this.get_property("paths");
         Ok::<_, phper::Error>(paths.clone())
    });
    event.add_method("getKind", Visibility::Public, |this: &mut StateObj<()>, _: &mut [ZVal]| {
        let kind = this.get_property("kind");
         Ok::<_, phper::Error>(kind.clone())
    });

    let mut watcher = ClassEntity::new("FsNotify\\RecommendedWatcher");
    watcher.add_property("map", Visibility::Private, ());

    watcher.add_method("__construct", Visibility::Public, |this, _arguments| {
        this.set_property("map", ZArray::new());
        Ok::<_, Infallible>(())
    });

    watcher.add_method("add", Visibility::Public, |this, arguments| {
        let path = arguments[0].expect_z_str()?;

        let recursive = match arguments.get(1) {
            Some(recursive) => recursive.expect_bool()?,
            None => true,
        };

        let map = this.get_mut_property("map").expect_mut_z_arr()?;
        map.insert(path, ZVal::from(recursive));

        Ok::<_, phper::Error>(())
    })
        .argument(Argument::by_val("path"))
        .argument(Argument::by_val_optional("recursive"));

    watcher.add_method("remove", Visibility::Public, |this, arguments| {
        let path = arguments[0].expect_z_str()?;

        let map = this.get_mut_property("map").expect_mut_z_arr()?;
        map.remove(path);

        Ok::<_, phper::Error>(())
    })
        .argument(Argument::by_val("path"));

    watcher.add_method("watch", Visibility::Public, |this, arguments| {
        let handler = arguments.get_mut(0).unwrap();
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
        let map = this.get_mut_property("map").expect_mut_z_arr()?;

        for (k, v) in map.iter() {
            let path = match k {
                IterKey::ZStr(path) => PathBuf::from(path.to_str()?),
                _ => continue
            };

            let recursive = match v.expect_bool()? {
                true => RecursiveMode::Recursive,
                false => RecursiveMode::NonRecursive,
            };

            watcher.watch(&path, recursive).unwrap();
        }

        for res in rx {
            match res {
                Ok(event) => {
                    let mut php_event = ClassEntry::from_globals("FsNotify\\Event")?.init_object()?;

                    let paths_as_str = event.paths.iter()
                        .map(|p| ZVal::from(p.to_str()))
                        .collect::<Vec<ZVal>>();

                    let mut arr = ZArray::new();
                    for path_as_str in paths_as_str.into_iter() {
                        arr.insert(InsertKey::NextIndex, path_as_str);
                    }

                    let php_kind = match event.kind {
                        EventKind::Any => "any",
                        EventKind::Access(AccessKind::Any) => "access-any",
                        EventKind::Access(AccessKind::Read) => "access-read",
                        EventKind::Access(AccessKind::Open(AccessMode::Any)) => "access-open-any",
                        EventKind::Access(AccessKind::Open(AccessMode::Execute)) => "access-open-execute",
                        EventKind::Access(AccessKind::Open(AccessMode::Read)) => "access-open-read",
                        EventKind::Access(AccessKind::Open(AccessMode::Write)) => "access-open-write",
                        EventKind::Access(AccessKind::Open(AccessMode::Other)) => "access-open-other",
                        EventKind::Access(AccessKind::Close(AccessMode::Any)) => "access-close-any",
                        EventKind::Access(AccessKind::Close(AccessMode::Execute)) => "access-close-execute",
                        EventKind::Access(AccessKind::Close(AccessMode::Read)) => "access-close-read",
                        EventKind::Access(AccessKind::Close(AccessMode::Write)) => "access-close-write",
                        EventKind::Access(AccessKind::Close(AccessMode::Other)) => "access-close-other",
                        EventKind::Access(AccessKind::Other) => "access-other",
                        EventKind::Create(CreateKind::Any) => "create-any",
                        EventKind::Create(CreateKind::File) => "create-file",
                        EventKind::Create(CreateKind::Folder) => "create-folder",
                        EventKind::Create(CreateKind::Other) => "create-other",
                        EventKind::Modify(ModifyKind::Any) => "modify-any",
                        EventKind::Modify(ModifyKind::Data(DataChange::Any)) => "modify-data-any",
                        EventKind::Modify(ModifyKind::Data(DataChange::Size)) => "modify-data-size",
                        EventKind::Modify(ModifyKind::Data(DataChange::Content)) => "modify-data-content",
                        EventKind::Modify(ModifyKind::Data(DataChange::Other)) => "modify-data-other",
                        EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any)) => "modify-metadata-any",
                        EventKind::Modify(ModifyKind::Metadata(MetadataKind::AccessTime)) => "modify-metadata-access-time",
                        EventKind::Modify(ModifyKind::Metadata(MetadataKind::WriteTime)) => "modify-metadata-write-time",
                        EventKind::Modify(ModifyKind::Metadata(MetadataKind::Permissions)) => "modify-metadata-permissions",
                        EventKind::Modify(ModifyKind::Metadata(MetadataKind::Ownership)) => "modify-metadata-ownership",
                        EventKind::Modify(ModifyKind::Metadata(MetadataKind::Extended)) => "modify-metadata-extended",
                        EventKind::Modify(ModifyKind::Metadata(MetadataKind::Other)) => "modify-metadata-other",
                        EventKind::Modify(ModifyKind::Name(RenameMode::Any)) => "modify-rename-any",
                        EventKind::Modify(ModifyKind::Name(RenameMode::To)) => "modify-rename-to",
                        EventKind::Modify(ModifyKind::Name(RenameMode::From)) => "modify-rename-from",
                        EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => "modify-rename-both",
                        EventKind::Modify(ModifyKind::Name(RenameMode::Other)) => "modify-rename-other",
                        EventKind::Modify(ModifyKind::Other) => "modify-other",
                        EventKind::Remove(RemoveKind::Any) => "remove-any",
                        EventKind::Remove(RemoveKind::File) => "remove-file",
                        EventKind::Remove(RemoveKind::Folder) => "remove-folder",
                        EventKind::Remove(RemoveKind::Other) => "remove-other",
                        EventKind::Other => "other",
                    };

                    let paths = php_event.get_mut_property("paths");
                    *paths = arr.into();

                    let kind = php_event.get_mut_property("kind");
                    *kind = php_kind.into();

                    handler.call([ZVal::from(php_event)])?;
                },
                Err(_) => {
                },
            }
        }

        Ok::<_, phper::Error>(())
    })
        .argument(Argument::by_val("handle"));

    module.add_class(watcher);
    module.add_class(event);

    module
}