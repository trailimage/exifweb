import { JsonLD, LinkData } from '@toba/json-ld';
import { forVideo } from './json-ld';

export class VideoInfo implements LinkData<JsonLD.VideoObject> {
   id: string;
   width: number = 0;
   height: number = 0;

   constructor(id: string, width: number, height: number) {
      this.id = id;
      this.width = width;
      this.height = height;
   }

   get empty() {
      return this.width === 0 || this.height === 0;
   }

   jsonLD(): JsonLD.VideoObject {
      return forVideo(this)!;
   }
}
