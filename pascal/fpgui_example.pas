program fpgui_example;

{$mode objfpc}{$H+}

uses
  SysUtils,
  fpg_base,
  fpg_main,
  fpg_form,
  fpg_button,
  fpg_edit,
  fpg_label,
  fpg_panel,
  fpg_combobox;

type
  TMainForm = class(TfpgForm)
  private
    {@VFD_HEAD_BEGIN: MainForm}
    btnDraw: TfpgButton;
    btnClear: TfpgButton;
    edtText: TfpgEdit;
    lblStatus: TfpgLabel;
    cmbShape: TfpgComboBox;
    cmbColor: TfpgComboBox;
    pnlCanvas: TfpgPanel;
    {@VFD_HEAD_END: MainForm}
    FShapes: array of record
      X, Y: Integer;
      Shape: Integer;
      Color: TfpgColor;
    end;
    procedure btnDrawClick(Sender: TObject);
    procedure btnClearClick(Sender: TObject);
    procedure pnlCanvasPaint(Sender: TObject);
  public
    procedure AfterCreate; override;
  end;

{@VFD_NEWFORM_IMPL}

procedure TMainForm.btnDrawClick(Sender: TObject);
var
  idx: Integer;
begin
  // Add new shape
  idx := Length(FShapes);
  SetLength(FShapes, idx + 1);
  
  FShapes[idx].X := Random(pnlCanvas.Width - 40) + 20;
  FShapes[idx].Y := Random(pnlCanvas.Height - 40) + 20;
  FShapes[idx].Shape := cmbShape.FocusItem;
  
  case cmbColor.FocusItem of
    0: FShapes[idx].Color := clRed;
    1: FShapes[idx].Color := clBlue;
    2: FShapes[idx].Color := clGreen;
    3: FShapes[idx].Color := clYellow;
    else FShapes[idx].Color := clBlack;
  end;
  
  lblStatus.Text := 'Drew ' + cmbShape.Text + ' at (' + 
    IntToStr(FShapes[idx].X) + ', ' + IntToStr(FShapes[idx].Y) + ') - Total: ' + 
    IntToStr(Length(FShapes));
  
  pnlCanvas.Invalidate;
end;

procedure TMainForm.btnClearClick(Sender: TObject);
begin
  SetLength(FShapes, 0);
  lblStatus.Text := 'Canvas cleared';
  pnlCanvas.Invalidate;
end;

procedure TMainForm.pnlCanvasPaint(Sender: TObject);
var
  i: Integer;
  cv: TfpgCanvas;
begin
  cv := pnlCanvas.Canvas;
  
  // Draw background
  cv.Clear(clWhite);
  
  // Draw grid
  cv.SetColor(clSilver);
  for i := 0 to pnlCanvas.Width div 20 do
    cv.DrawLine(i * 20, 0, i * 20, pnlCanvas.Height);
  for i := 0 to pnlCanvas.Height div 20 do
    cv.DrawLine(0, i * 20, pnlCanvas.Width, i * 20);
  
  // Draw all shapes
  for i := 0 to High(FShapes) do
  begin
    cv.SetColor(FShapes[i].Color);
    
    case FShapes[i].Shape of
      0: begin // Circle
        cv.FillArc(FShapes[i].X - 20, FShapes[i].Y - 20, 40, 40, 0, 360);
        cv.SetColor(clBlack);
        cv.DrawArc(FShapes[i].X - 20, FShapes[i].Y - 20, 40, 40, 0, 360);
      end;
      1: begin // Rectangle
        cv.FillRectangle(FShapes[i].X - 20, FShapes[i].Y - 20, 40, 40);
        cv.SetColor(clBlack);
        cv.DrawRectangle(FShapes[i].X - 20, FShapes[i].Y - 20, 40, 40);
      end;
      2: begin // Line
        cv.SetLineStyle(2, lsSolid);
        cv.DrawLine(FShapes[i].X, FShapes[i].Y, 
                    FShapes[i].X + 40, FShapes[i].Y + 40);
      end;
    end;
  end;
end;

procedure TMainForm.AfterCreate;
begin
  {%region 'Auto-generated GUI code' -fold}
  {@VFD_BODY_BEGIN: MainForm}
  Name := 'MainForm';
  SetPosition(300, 200, 600, 500);
  WindowTitle := 'fpGUI Drawing Example';
  Hint := '';

  // Status label
  lblStatus := TfpgLabel.Create(self);
  with lblStatus do
  begin
    Name := 'lblStatus';
    SetPosition(10, 10, 580, 20);
    FontDesc := '#Label1';
    Hint := '';
    Text := 'Click Draw to add shapes';
  end;

  // Shape combo
  cmbShape := TfpgComboBox.Create(self);
  with cmbShape do
  begin
    Name := 'cmbShape';
    SetPosition(10, 40, 120, 24);
    FontDesc := '#List';
    Hint := '';
    Items.Add('Circle');
    Items.Add('Rectangle');
    Items.Add('Line');
    FocusItem := 0;
  end;

  // Color combo
  cmbColor := TfpgComboBox.Create(self);
  with cmbColor do
  begin
    Name := 'cmbColor';
    SetPosition(140, 40, 120, 24);
    FontDesc := '#List';
    Hint := '';
    Items.Add('Red');
    Items.Add('Blue');
    Items.Add('Green');
    Items.Add('Yellow');
    FocusItem := 0;
  end;

  // Text input
  edtText := TfpgEdit.Create(self);
  with edtText do
  begin
    Name := 'edtText';
    SetPosition(270, 40, 200, 24);
    FontDesc := '#Edit1';
    Hint := '';
    TabOrder := 1;
    Text := 'Enter text here...';
  end;

  // Draw button
  btnDraw := TfpgButton.Create(self);
  with btnDraw do
  begin
    Name := 'btnDraw';
    SetPosition(480, 40, 80, 24);
    Text := 'Draw';
    FontDesc := '#Label1';
    Hint := '';
    ImageName := '';
    TabOrder := 2;
    OnClick := @btnDrawClick;
  end;

  // Clear button
  btnClear := TfpgButton.Create(self);
  with btnClear do
  begin
    Name := 'btnClear';
    SetPosition(480, 70, 80, 24);
    Text := 'Clear';
    FontDesc := '#Label1';
    Hint := '';
    ImageName := '';
    TabOrder := 3;
    OnClick := @btnClearClick;
  end;

  // Canvas panel
  pnlCanvas := TfpgPanel.Create(self);
  with pnlCanvas do
  begin
    Name := 'pnlCanvas';
    SetPosition(10, 100, 580, 390);
    FontDesc := '#Label1';
    Hint := '';
    Style := bsFlat;
    OnPaint := @pnlCanvasPaint;
  end;

  {@VFD_BODY_END: MainForm}
  {%endregion}
  
  SetLength(FShapes, 0);
end;

procedure MainProc;
var
  frm: TMainForm;
begin
  fpgApplication.Initialize;
  frm := TMainForm.Create(nil);
  try
    frm.Show;
    fpgApplication.Run;
  finally
    frm.Free;
  end;
end;

begin
  MainProc;
end.
