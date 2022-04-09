import { Component, OnInit } from '@angular/core';
import { MongodbService } from './mongodb.service';

import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { MainModalComponent } from './main-modal/main-modal.component';
import * as wasm from "rust-api";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'my-app';

  constructor(private mongodb: MongodbService, private modalService: NgbModal) {}

  ngOnInit() {
    let modal = new MainModalComponent(this.modalService);
    modal.open();
    // wasm.greet();
  }

  async onSubmitUsername(username: string) {
    let user = await this.mongodb.getProfile(username);
    window.alert(`Username: ${user.username}`);
  }
}
