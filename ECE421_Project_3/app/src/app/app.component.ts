import { Component, OnInit } from '@angular/core';
import { MongodbService } from './mongodb.service';
import * as wasm from "rust-api";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'my-app';

  constructor(private mongodb: MongodbService) {}

  ngOnInit() {
    wasm.greet();
  }

  async onSubmitUsername(username: string) {
    let user = await this.mongodb.getProfile(username);
    window.alert(`Username: ${user.username}`);
  }
}
