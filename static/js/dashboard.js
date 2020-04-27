$("#btn-stockin").click(() => {
  $("#modal-stockin").modal("toggle")
})

$("#btn-stockout").click(() => {
  $("#modal-stockout").modal("toggle")
})

$("#btn-stockmove").click(() => {
  $("#modal-stockmove").modal("toggle")
})

$("#modal-stockin-btn-add-field").click(() => {
  const n = $("#modal-stockin-body .field").length;

  $("#modal-stockin-body").append(`
<div class="row no-gutters mt-3 field" id="modal-stockin-field${n + 1}">
  <div class="col">
    <label for="modal-stockin-input-field${n + 1}-key" class="sr-only">Key</label>
    <input class="form-control key" id="modal-stockin-input-field${n + 1}-key" placeholder="key" required>
    <div class="my-2"></div>
    <label for="modal-stockin-input-field${n + 1}-value" class="sr-only">Value</label>
    <input class="form-control value" id="modal-stockin-input-field${n + 1}-value" placeholder="value" required>
  </div>
  <div class="col-auto ml-2 d-flex align-items-stretch">
    <button class="btn btn-block btn-outline-danger" id="modal-stockin-btn-remove-field${n + 1}" aria-label="Remove this field" onclick="removeField('#modal-stockin-field${n + 1}')">&times;</button>
  </div>
</div>`)
})

const removeField = (f) => {
  $(f).detach();
}

$("#modal-stockin-btn-submit").click(() => {
  const validity = $
    .map($("#modal-stockin-body input"), (v) => v.validity.valid)
    .reduce((a, v) => a & v, true)
  if (!validity)
    return

  const keys = $.map($("#modal-stockin-body .real-key"), (v) => v.textContent)
    .concat($.map($("#modal-stockin-body .key"), (v) => v.value))
  const values = $.map($("#modal-stockin-body .value"), (v) => v.value)
  const object = Object.fromEntries(keys.map((_, i) => [keys[i], values[i]]))

  $.ajax({
    method: "POST",
    url: "/product",
    contentType: "application/json",
    data: JSON.stringify(object)
  }).done(() => {
    $("#modal-stockin").modal("hide")
    location.reload()
  })
})

$("#modal-stockout-btn-submit").click(() => {
  const validity = $
    .map($("#modal-stockout-body input"), (v) => v.validity.valid)
    .reduce((a, v) => a & v, true)
  if (!validity)
    return

  const pn = $("#modal-stockout-input-pn")[0].value
  const recipient = $("#modal-stockout-input-recipient")[0].value
  const object = {"in": recipient}

  $.getJSON(`/product/${pn}`, (product) => {
    product.in = recipient
    delete product.on

    // Must be a string, see src/product/structure.rs
    product.amount = "0"
  }).done((product) => {
    $.ajax({
      method: "PUT",
      url: `/product/${pn}`,
      contentType: "application/json",
      data: JSON.stringify(product)
    }).done(() => {
      $("#modal-stockout").modal("hide")
      location.reload()
    })
  })
})

$("#modal-stockmove-btn-submit").click(() => {
  const validity = $
    .map($("#modal-stockmove-body input"), (v) => v.validity.valid)
    .reduce((a, v) => a & v, true)
  if (!validity)
    return

  const pn = $("#modal-stockmove-input-pn")[0].value
  const to = $("#modal-stockmove-input-recipient")[0].value
  const object = {"in": to}

  $.ajax({
    method: "PATCH",
    url: `/product/${pn}`,
    contentType: "application/json",
    data: JSON.stringify(object)
  }).done(() => {
    $("#modal-stockmove").modal("hide")
    location.reload()
  })
})
