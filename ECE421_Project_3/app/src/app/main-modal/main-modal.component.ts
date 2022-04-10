import { Component, OnInit } from '@angular/core';
import { NgbModalConfig, NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';

@Component({
  selector: 'modal-content',
  templateUrl: './main-modal.component.html',
  styleUrls: ['./main-modal.component.css'],
})
export class NgbdModalContent {
  constructor(public activeModal: NgbActiveModal ) {}
}

@Component({
  selector: 'app-main-modal',
  templateUrl: './main-modal.component.html',
  styleUrls: ['./main-modal.component.css'],
  providers: [NgbModalConfig, NgbModal]
})
export class MainModalComponent implements OnInit {

  constructor(private modalService: NgbModal) {}

  ngOnInit() {
    this.modalService.open(this);
  }

  open() {
    // this.modalService.open(NgbdModalContent);
  }
}
