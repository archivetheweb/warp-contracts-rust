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
  | ArchiveRequestByIDResult;

export interface ArchivesByURL {
  count: number;
  url: string;
}
export interface ArchivesByURLResult {
  archives: ArchiveSubmission[];
}
export interface ArchiveSubmission {
  archiveRequestId: string;
  arweaveTx: string;
  fullUrl: string;
  options: ArchiveOptions;
  size: number;
  timestamp: number;
  uploaderAddress: string;
}
export interface ArchiveOptions {
  depth: number;
  domainOnly: boolean;
}
export interface ArchiveRequestsFor {
  address: string;
}
export interface ArchiveRequestsForResult {
  archivesRequests: ArchiveRequest[];
}
export interface ArchiveRequest {
  crawlOptions: CrawlOptions;
  endTimestamp: number;
  frequency: string;
  id: string;
  requestedBy: string;
  startTimestamp: number;
  uploaderAddress: string;
}
export interface CrawlOptions {
  depth: number;
  domainOnly: boolean;
  urls: string[];
}
export interface ArchiveRequestByID {
  archiveId: string;
}
export interface ArchiveRequestByIDResult {
  archivesRequest?: ArchiveRequest | null;
}
