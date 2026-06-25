export function renderReadingView(container: HTMLElement) {
  container.innerHTML = `
    <div class="therapy-view">
      <button id="btn-back-reading" class="btn-back" data-i18n="btn_back">← Volver al Menú</button>
      <hr style="border:0; border-top:1px solid #eee; margin:15px 0" />
      
      <div class="main-actions">
        <button id="btn-start-reading" style="background-color: #9b59b6; color: white" data-i18n="btn_start_reading">
          Abrir Ventana
        </button>
        <button id="btn-stop-reading" style="background-color: #e74c3c; color: white" data-i18n="btn_stop_reading">
          Cerrar Ventana
        </button>
      </div>

      <div class="control-section">
        <textarea id="reading-input" style="width:100%; height:250px; padding:15px; border-radius:8px; border:1px solid #ddd; resize:vertical; box-sizing:border-box; font-family:inherit;" data-i18n="reading_input_placeholder" placeholder="Pega aquí el contenido..."></textarea>
      </div>
    </div>
  `;
}
