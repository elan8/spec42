use crate::views::dto::{
    range_to_dto, SysmlLibrarySearchItemDto, SysmlLibrarySearchPackageDto,
    SysmlLibrarySearchSourceDto,
};
use language_service::{LibrarySearchItem, LibrarySearchPackage, LibrarySearchSource};

fn to_dto_item(item: LibrarySearchItem) -> SysmlLibrarySearchItemDto {
    SysmlLibrarySearchItemDto {
        name: item.name,
        kind: item.kind,
        container: item.container,
        uri: item.uri,
        range: range_to_dto(item.range),
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
