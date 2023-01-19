/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type WriteAction = RegisterUploader | RequestArchiving | SubmitArchive | DeleteArchiveRequest | Evolve;

export interface RegisterUploader {
  friendlyName: string;
}
export interface RequestArchiving {
  crawlOptions: CrawlOptions;
  endTimestamp: number;
  frequency: string;
  startTimestamp: number;
  uploaderAddress: string;
}
export interface CrawlOptions {
  depth: number;
  domainOnly: boolean;
  urls: string[];
}
export interface SubmitArchive {
  archivingRequestId: string;
  arweaveTx: string;
  fullUrl: string;
  info: CrawlOptions;
  size: number;
  timestamp: number;
  uploaderAddress: string;
}
export interface DeleteArchiveRequest {
  archiveId: string;
}
export interface Evolve {
  value: string;
}
