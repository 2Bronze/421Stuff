import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../environments/environment';
@Injectable({
  providedIn: 'root'
})
export class MongodbService {

  constructor(private http: HttpClient) {}

  private async request(method: string, url: string, data?: any) {
    const result = this.http.request(method, url,  {
      body: data,
      responseType: 'json',
      observe: 'body',
    });
    return new Promise<any>((res, rej) => {
      result.subscribe(res, rej);
    });
  }


  getProfile(username: string) {
    return this.request('GET', `${environment.server}/profile/${username}`);
  }
}
