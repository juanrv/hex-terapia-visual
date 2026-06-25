export function renderOverlayView(container: HTMLElement) {
  container.innerHTML = `
    <div class="therapy-view">
      <button id="btn-back-overlay" class="btn-back" data-i18n="btn_back">← Volver al Menú</button>
      <hr style="border:0; border-top:1px solid #eee; margin:15px 0" />
      
      <div class="main-actions">
        <button id="btn-start" data-i18n="btn_start">Iniciar Terapia</button>
        <button id="btn-stop" data-i18n="btn_stop">Detener Terapia</button>
        <button id="btn-reset" data-i18n="btn_reset">Restablecer</button>
      </div>
      
      <div class="control-section">
        <h3 data-i18n="layout_label">Disposición (Layout)</h3>
        <select id="layout-select" class="layout-selector">
          <option value="Vertical" data-i18n="layout_vertical">Vertical</option>
          <option value="Horizontal" data-i18n="layout_horizontal">Horizontal</option>
          <option value="Checkerboard" data-i18n="layout_checkerboard">Ajedrez</option>
          <option value="Vertical4Columns" data-i18n="layout_vertical4">Vertical (4 Columnas)</option>
        </select>
      </div>
      
      <div class="control-section">
        <div id="zones-container"></div>
      </div>
    </div>
  `;
}
