import { is } from '@toba/tools';
import { Token } from '@toba/oauth';
import { IncomingMessage } from 'http';
import {
   TrackFeatures,
   loadSource,
   MapConfig,
   config as mapConfig
} from '@toba/map';
import { ProviderConfig } from './config';
import { EXIF, Photo, Post, PhotoBlog, config } from './index';
import { FeatureCollection, GeometryObject } from 'geojson';
import { Writable } from 'stream';

/**
 * Methods that provide model data.
 */
export abstract class DataProvider<T> {
   config: T;
   requiresAuthentication: boolean = true;
   isAuthenticated: boolean = false;

   constructor(baseConfig?: T) {
      if (baseConfig !== undefined) {
         this.config = baseConfig;
      }
   }

   /**
    * Provider URL to load when user needs to be authenticated. The URL will
    * call back to an endpoint that should use `getAccessToken`.
    */
   abstract authorizationURL(): Promise<string>;

   /**
    * Parse authorization callback to make call that will generate an access
    * token.
    */
   abstract getAccessToken(req: IncomingMessage): Promise<Token>;

   /**
    * Apply configuration.
    */
   configure(newConfig: Partial<T>) {
      if (is.value(this.config)) {
         Object.assign(this.config, newConfig);
      } else {
         this.config = newConfig as T;
      }
   }
}

/**
 * Methods to load post-related data.
 */
export abstract class PostProvider<T> extends DataProvider<T> {
   /** Populate categories and post summaries in current blog instance. */
   abstract photoBlog(async?: boolean): Promise<PhotoBlog>;
   /** Retrieve EXIF for single photo. */
   abstract exif(photoID: string): Promise<EXIF>;
   /** Find ID of Post that contains photo with given ID. */
   abstract postIdWithPhotoId(photoID: string): Promise<string | null>;
   abstract photosWithTags(tags: string | string[]): Promise<Photo[]>;
   abstract postInfo(p: Post): Promise<Post>;
   abstract postPhotos(p: Post): Promise<Photo[]>;
}

/**
 * Methods to load map-related data like GPX tracks.
 */
export abstract class MapProvider<T extends MapConfig> extends DataProvider<T> {
   abstract track(postKey: string): Promise<TrackFeatures>;
   /**
    * Send GPX data for post to a writable stream (usually HTTP response). The
    * `Promise` is resolved when the stream `end` event fires.
    */
   abstract gpx(postKey: string, stream: Writable): Promise<void>;

   /**
    * @param baseConfig Configuration for provider API as well as the map module
    */
   constructor(baseConfig?: T) {
      super(baseConfig);

      if (baseConfig !== undefined) {
         Object.assign(mapConfig, baseConfig);
      }
   }

   /**
    * Apply new configuration values to the provider API as well as the map
    * module. Technically, the values will be duplicated to both targets but
    * unused values will simply be ignored.
    */
   configure(newConfig: Partial<T>) {
      super.configure(newConfig);
      Object.assign(mapConfig, newConfig);
   }

   /**
    * @param sourceKey configured `MapSource` key
    */
   source(
      sourceKey: string
   ): Promise<FeatureCollection<GeometryObject> | null> {
      return loadSource(sourceKey);
   }
}

/**
 * Methods to load videos associated with a post.
 */
export abstract class VideoProvider<T> extends DataProvider<T> {}

/**
 * Return configured post provider or throw a reference error.
 */
export const ensurePostProvider = (): PostProvider<any> =>
   ensureProvider('post');

/**
 * Return configured map provider or throw a reference error.
 */
export const ensureMapProvider = (): MapProvider<any> => ensureProvider('map');

/**
 * Return configured video provider or throw a reference error.
 */
export const ensureVideoProvider = (): VideoProvider<any> =>
   ensureProvider('video');

/**
 * Return provider or throw a reference error.
 */
function ensureProvider<K extends keyof ProviderConfig>(key: K) {
   if (!is.value(config.providers![key])) {
      throw new ReferenceError(key + ' provider is undefined');
   }
   return config.providers[key]!;
}
