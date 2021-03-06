import { geoJSON, IMappable } from "@toba/map";
import { PhotoSize, EXIF, PostProvider } from "./index";
import { ensurePostProvider } from "./providers";

/**
 * Middle value or average of the two middle values among a list of numbers.
 * @param values Sorted numbers
 */
export const median = (...values: number[]) => {
  const half = Math.floor(values.length / 2);
  return values.length % 2 !== 0
    ? values[half]
    : (values[half - 1] + values[half]) / 2.0;
};

export interface Limits {
  min: number;
  max: number;
}

/**
 * Calculate Tukey fence values for a range of numbers. Values outside the
 * range are considered outliers.
 *
 * @param distance Constant used to calculate fence. Tukey proposed `1.5` for an
 * "outlier" and `3` for "far out". This method defaults to `3` if no value is
 * given.
 *
 * @see http://sphweb.bumc.bu.edu/otlt/MPH-Modules/BS/BS704_SummarizingData/BS704_SummarizingData7.html
 */
export const boundary = (values: number[], distance = 3): Limits | null => {
  if (!is.array(values) || values.length === 0) return null;

  // sort lowest to highest
  values.sort((d1, d2) => d1 - d2);

  const half = Math.floor(values.length / 2);
  /** first quartile */
  const q1 = median(...values.slice(0, half));
  /** third quartile */
  const q3 = median(...values.slice(half));
  /** interquartile range: range of the middle 50% of the data */
  const range = q3 - q1;

  return {
    min: (q1 - range * distance) as number,
    max: (q3 + range * distance) as number
  };
};

export class Photo implements IMappable<GeoJSON.Point> {
  /** Provider photo ID */
  id: string;
  /** Position of photo within post. */
  index: number;
  sourceUrl: string;
  title: string;
  description?: string;
  /** Tags applied to the photo. */
  tags: Set<string> = new Set();
  dateTaken?: Date;
  latitude?: number;
  longitude?: number;
  /** Whether this is the post's main photo. */
  primary: boolean = false;
  size: { [key: string]: PhotoSize } = {};
  /** Size at which to preview the photo such as in search results. */
  preview: PhotoSize;
  /** Normal photo size shown within post. */
  normal: PhotoSize;
  /** Size shown when post photo is clicked for enlargmenet. */
  big: PhotoSize;

  private _exif: EXIF;

  /**
   * Whether taken date is an outlier compared to other photos in the same
   * post. Outliers may be removed from mini-maps so the maps aren't overly
   * zoomed-out to accomodate contextual photos taken days before or after
   * the main post.
   *
   * @see http://www.wikihow.com/Calculate-Outliers
   */
  outlierDate?: boolean;

  constructor(id: string, index: number) {
    this.id = id;
    this.index = index;
  }

  private get load(): PostProvider<any> {
    return ensurePostProvider();
  }

  /**
   * Comma-delimited list of all tags applied to the photo.
   */
  get tagList(): string {
    return Array.from(this.tags).join(",");
  }

  async EXIF(): Promise<EXIF> {
    if (this._exif === null) {
      this._exif = await this.load.exif(this.id);
    }
    return this._exif;
  }

  /**
   * GeoJSON point constructor expect requires populated coordinate array or
   * `null`.
   */
  private get coordinate(): [number, number] | null {
    return this.longitude !== undefined && this.latitude !== undefined
      ? [this.longitude, this.latitude]
      : null;
  }

  /**
   * Generate GeoJSON for photo feature.
   *
   * @param partKey Optional series part that photo post belongs to, used to
   * generate link from map info box back to post URL
   */
  geoJSON(partKey?: string): GeoJSON.Feature<GeoJSON.Point> {
    const properties: MapPhoto = { url: this.size.preview.url };

    if (partKey !== undefined) {
      // implies GeoJSON for single post
      properties.title = this.title;
      properties.partKey = partKey;
    }
    return {
      type: geoJSON.Type.Feature,
      properties,
      geometry: geoJSON.geometry(geoJSON.Type.Point, this.coordinate)
    } as GeoJSON.Feature<GeoJSON.Point>;
  }
}

/**
 * Simplistic outlier calculation identifies photos that are likely not part of
 * the main sequence.
 *
 * @see https://en.wikipedia.org/wiki/Outlier
 * @see http://www.wikihow.com/Calculate-Outliers
 */
export function identifyOutliers(photos: Photo[]) {
  const fence = boundary(
    photos
      .filter(p => p.dateTaken !== undefined)
      .map(p => p.dateTaken!.getTime())
  );

  if (fence !== null) {
    for (const p of photos) {
      if (p.dateTaken === undefined) {
        continue;
      }
      const d = p.dateTaken.getTime();
      if (d > fence.max || d < fence.min) {
        p.outlierDate = true;
      }
    }
  }
}

/**
 * GeoJSON properties for photos.
 */
export interface MapPhoto {
  url?: string;
  title?: string;
  partKey?: string;
  /** Distance from clicked cluster */
  distance?: number;
}
