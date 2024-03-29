/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type View =
  | ArchivesByURL
  | ArchivesByURLResult
  | ArchiveRequestsFor
  | ArchiveRequestsForResult
  | ArchiveRequestByID
  | ArchiveRequestByIDResult
  | Archives
  | ArchivesResult
  | ArchivesByURLAndTimestamp
  | ArchivesByURLAndTimestampResult;
export type CrawlType = 'domainOnly' | 'domainWithPageLinks' | 'domainAndLinks';

export interface ArchivesByURL {
  count: number;
  url: string;
}
export interface ArchivesByURLResult {
  archives: ArchivesByURLInfo;
}
export interface ArchivesByURLInfo {
  archivedInfo: ArchiveSubmission[];
  lastArchivedTimestamp: number;
  screenshotTx: string;
  title: string;
  url: string;
}
export interface ArchiveSubmission {
  archiveRequestId: string;
  arweaveTx: string;
  fullUrl: string;
  options: ArchiveOptions;
  screenshotTx: string;
  size: number;
  timestamp: number;
  title: string;
  uploaderAddress: string;
}
export interface ArchiveOptions {
  crawlType: CrawlType;
  depth: number;
}
export interface ArchiveRequestsFor {
  address: string;
}
export interface ArchiveRequestsForResult {
  archivesRequests: ArchiveRequest[];
}
export interface ArchiveRequest {
  endTimestamp: number;
  frequency: string;
  id: string;
  latestArchivedTimestamp: number;
  options: ArchiveRequestOptions;
  requestedBy: string;
  startTimestamp: number;
  uploaderAddress: string;
}
export interface ArchiveRequestOptions {
  crawlType: CrawlType;
  depth: number;
  urls: string[];
}
export interface ArchiveRequestByID {
  archiveId: string;
}
export interface ArchiveRequestByIDResult {
  archivesRequest?: ArchiveRequest | null;
}
export interface Archives {}
export interface ArchivesResult {
  archives: ArchiveInfo[];
}
export interface ArchiveInfo {
  archivedCount: number;
  lastArchivedTimestamp: number;
  screenshotTx: string;
  title: string;
  url: string;
}
export interface ArchivesByURLAndTimestamp {
  timestamp: number;
  url: string;
}
export interface ArchivesByURLAndTimestampResult {
  archive: ArchiveSubmission;
}
