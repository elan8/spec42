use crate::common::text_span::to_core_range;
use crate::views::dto::{
    SysmlLibrarySearchItemDto, SysmlLibrarySearchPackageDto, SysmlLibrarySearchSourceDto,
};
use crate::workspace::library_search::{
    LibrarySearchItem, LibrarySearchPackage, LibrarySearchSource,
};
use sysml_model::range_to_dto;

fn to_dto_item(item: LibrarySearchItem) -> SysmlLibrarySearchItemDto {
    SysmlLibrarySearchItemDto {
        name: item.name,
        kind: item.kind,
        container: item.container,
        uri: item.uri,
        range: range_to_dto(to_core_range(item.range)),
        score: item.score,
        source: item.source,
        path: item.path,
    }
}

fn to_dto_package(package: LibrarySearchPackage) -> SysmlLibrarySearchPackageDto {
    SysmlLibrarySearchPackageDto {
        name: package.name,
        path: package.path,
        source: package.source,
        symbols: package.symbols.into_iter().map(to_dto_item).collect(),
    }
}

pub(crate) fn to_dto_sources(
    sources: Vec<LibrarySearchSource>,
) -> Vec<SysmlLibrarySearchSourceDto> {
    sources
        .into_iter()
        .map(|source| SysmlLibrarySearchSourceDto {
            source: source.source,
            packages: source.packages.into_iter().map(to_dto_package).collect(),
        })
        .collect()
}
