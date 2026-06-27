import { translate } from "../../core/localization/i18n";

export function renderZoneControls(
  config: any,
  onUpdate: (
    index: number,
    newColor: string | null,
    newOpacity: number | null,
  ) => void,
) {
  const container = document.getElementById("zones-container");
  if (!container) return;

  container.innerHTML = "";

  config.zones_config.forEach((zone: any, index: number) => {
    const card = document.createElement("div");
    card.className = "zone-card";

    // Titulo de la zona
    const title = document.createElement("h4");
    title.innerText = `${translate("zone_title")} ${index + 1}`;
    card.appendChild(title);

    // Control de color
    const colorGroup = document.createElement("div");
    colorGroup.className = "control-group";

    const colorLabel = document.createElement("label");
    colorLabel.innerText = translate("color_label");

    const colorInput = document.createElement("input");
    colorInput.type = "color";
    colorInput.value = zone.color;

    colorInput.addEventListener("change", (e) => {
      const newColor = (e.target as HTMLInputElement).value;
      onUpdate(index, newColor, null);
    });

    colorGroup.appendChild(colorLabel);
    colorGroup.appendChild(colorInput);
    card.appendChild(colorGroup);

    // Control de opacidad
    const opacityGroup = document.createElement("div");
    opacityGroup.className = "control-group";

    const opacityLabel = document.createElement("label");
    opacityLabel.innerText = `${translate("opacity_label")} (${Math.round(zone.opacity * 100)}%)`;

    const opacityInput = document.createElement("input");
    opacityInput.type = "range";
    opacityInput.min = "0";
    opacityInput.max = "0.8";
    opacityInput.step = "0.01";
    opacityInput.value = zone.opacity;

    opacityInput.addEventListener("change", (e) => {
      const val = parseFloat((e.target as HTMLInputElement).value);
      opacityLabel.innerText = `${translate("opacity_label")} (${Math.round(val * 100)}%)`;
      onUpdate(index, null, val);
    });

    opacityGroup.appendChild(opacityLabel);
    opacityGroup.appendChild(opacityInput);
    card.appendChild(opacityGroup);

    container.appendChild(card);
  });
}
