import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IniciarMinecraftComponent } from './iniciar-minecraft.component';

describe('IniciarMinecraftComponent', () => {
  let component: IniciarMinecraftComponent;
  let fixture: ComponentFixture<IniciarMinecraftComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ IniciarMinecraftComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(IniciarMinecraftComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
