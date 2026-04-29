use crate::views::dto;
use crate::workspace::library_search::{
    LibrarySearchItem, LibrarySearchPackage, LibrarySearchSource,
};

fn to_dto_item(item: LibrarySearchItem) -> dto::SysmlLibrarySearchItemDto {
    dto::SysmlLibrarySearchItemDto {
        name: item.name,
        kind: item.kind,
        container: item.container,
        uri: item.uri,
        range: dto::range_to_dto(item.range),
        score: item.score,
        source: item.source,
        path: item.path,
    }
}

fn to_dto_package(package: LibrarySearchPackage) -> dto::SysmlLibrarySearchPackageDto {
    dto::SysmlLibrarySearchPackageDto {
        name: package.name,
        path: package.path,
        source: package.source,
        symbols: package.symbols.into_iter().map(to_dto_item).collect(),
    }
}

pub(crate) fn to_dto_sources(
    sources: Vec<LibrarySearchSource>,
) -> Vec<dto::SysmlLibrarySearchSourceDto> {
    sources
        .into_iter()
        .map(|source| dto::SysmlLibrarySearchSourceDto {
            source: source.source,
            packages: source.packages.into_iter().map(to_dto_package).collect(),
        })
        .collect()
}
