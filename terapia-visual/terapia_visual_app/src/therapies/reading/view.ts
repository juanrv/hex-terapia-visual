export function renderReadingView(container: HTMLElement) {
  container.innerHTML = `
    <div class="therapy-view">
      <button id="btn-back-reading" class="btn-back" data-i18n="btn_back">← Volver al Menú</button>
      <hr class="divider" />
      
      <div class="main-actions">
        <button id="btn-start-reading" class="btn-primary" data-i18n="btn_start_reading">
          Abrir Ventana
        </button>
        <button id="btn-stop-reading" class="btn-danger" data-i18n="btn_stop_reading">
          Cerrar Ventana
        </button>
      </div>

      <div class="control-section">
        <textarea id="reading-input" class="input-textarea" data-i18n="reading_input_placeholder" placeholder="Pega aquí el contenido..."></textarea>
      </div>
    </div>
  `;
}
