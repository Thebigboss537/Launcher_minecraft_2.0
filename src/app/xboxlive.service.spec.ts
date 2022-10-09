import { TestBed } from '@angular/core/testing';

import { XboxliveService } from './xboxlive.service';

describe('XboxliveService', () => {
  let service: XboxliveService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(XboxliveService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
