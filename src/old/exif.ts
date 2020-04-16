import { is } from '@toba/tools';
import { config } from './config';

/**
 * EXIF data for a photo.
 */
export class EXIF {
   artist?: string;
   compensation?: string;
   time?: string;
   fNumber: number = 0;
   focalLength: number | null;
   ISO: number = 0;
   lens?: string;
   model?: string;
   software?: string;
   /** Whether raw values have been formatted. */
   sanitized: boolean = false;

   sanitize(): EXIF {
      if (this.sanitized) {
         return this;
      }

      if (
         is.value<string>(this.artist) &&
         is.value<RegExp>(config.artistsToNormalize) &&
         config.artistsToNormalize.test(this.artist)
      ) {
         // only sanitize EXIF for photos shot by configured artists
         this.model = camera(this.model);
         this.lens = lens(this.lens, this.model);
         this.compensation = compensation(this.compensation);
         this.ISO = parseInt(this.ISO.toString());
         // don't show focal length for primes
         if (!numericRange.test(this.lens)) {
            this.focalLength = null;
         }
      }
      this.software = software(this.software);
      this.sanitized = true;

      return this;
   }
}

const numericRange = /\d\-\d/;

/**
 * Normalize camera name.
 */
const camera = (text?: string) =>
   is.empty(text)
      ? ''
      : text
           .replace('NIKON', 'Nikon')
           .replace('ILCE-7R', 'Sony α7ʀ')
           .replace('ILCE-7RM2', 'Sony α7ʀ II')
           .replace('Sony α7ʀM2', 'Sony α7ʀ II')
           .replace('VS980 4G', 'LG G2')
           .replace('XT1060', 'Motorola Moto X')
           .replace('TG-4', 'Olympus Tough TG-3');

/**
 * Normalize lens name.
 */
const lens = (text?: string, camera?: string) =>
   is.empty(text) || is.empty(camera)
      ? ''
      : text
           .replace(/FE 35mm.*/i, 'Sony FE 35mm ƒ2.8')
           .replace(/FE 55mm.*/i, 'Sony FE 55mm ƒ1.8')
           .replace(/FE 90mm.*/i, 'Sony FE 90mm ƒ2.8 OSS')
           .replace('58.0 mm f/1.4', 'Voigtländer Nokton 58mm ƒ1.4 SL II')
           .replace('14.0 mm f/2.8', 'Samyang 14mm ƒ2.8')
           .replace('50.0 mm f/1.4', 'Sigma 50mm ƒ1.4 EX DG')
           .replace(
              '35.0 mm f/2.0',
              /D700/.test(camera)
                 ? 'Zeiss Distagon T* 2/35 ZF.2'
                 : 'Nikkor 35mm ƒ2.0D'
           )
           .replace('100.0 mm f/2.0', 'Zeiss Makro-Planar T* 2/100 ZF.2')
           .replace('150.0 mm f/2.8', 'Sigma 150mm ƒ2.8 EX DG HSM APO')
           .replace('90.0 mm f/2.8', 'Tamron 90mm ƒ2.8 SP AF Di')
           .replace('24.0 mm f/3.5', 'Nikkor PC-E 24mm ƒ3.5D ED')
           .replace('14.0-24.0 mm f/2.8', 'Nikon 14–24mm ƒ2.8G ED')
           .replace('24.0-70.0 mm f/2.8', 'Nikon 24–70mm ƒ2.8G ED')
           .replace('17.0-55.0 mm f/2.8', 'Nikon 17–55mm ƒ2.8G')
           .replace('10.0-20.0 mm f/4.0-5.6', 'Sigma 10–20mm ƒ4–5.6 EX DC HSM')
           .replace(
              '1 NIKKOR VR 30-110mm f/3.8-5.6',
              'Nikkor 1 30–110mm ƒ3.8–5.6 VR'
           )
           .replace(
              '1 NIKKOR VR 10-30mm f/3.5-5.6',
              'Nikkor 1 10–30mm ƒ3.5–5.6 VR'
           )
           .replace(
              '18.0-200.0 mm f/3.5-5.6',
              'Nikkor 18–200mm ƒ3.5–5.6G ED VR'
           )
           .replace(
              /Voigtlander Heliar 15mm.*/i,
              'Voigtländer Heliar 15mm ƒ4.5 III'
           );

/**
 * Normalize software name.
 */
const software = (text?: string) =>
   is.empty(text)
      ? ''
      : text
           .replace('Photoshop Lightroom', 'Lightroom')
           .replace(/\s*\(Windows\)/, '');

/**
 * Normalize compensation value.
 */
const compensation = (text?: string | number) =>
   is.empty(text) || text == '0' ? 'No' : text.toString();
