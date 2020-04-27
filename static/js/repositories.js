$("#modal-newrepo-btn-add-field").click(() => {
  const n = $("#modal-newrepo-body .field").length;

  $("#modal-newrepo-body").append(`
<div class="row no-gutters mt-3 field" id="modal-newrepo-field${n + 1}">
  <div class="col">
    <label for="modal-newrepo-input-field${n + 1}-key" class="sr-only">Key</label>
    <input class="form-control key" id="modal-newrepo-input-field${n + 1}-key" placeholder="key" required>
    <div class="my-2"></div>
    <label for="modal-newrepo-input-field${n + 1}-value" class="sr-only">Value</label>
    <input class="form-control value" id="modal-newrepo-input-field${n + 1}-value" placeholder="value" required>
  </div>
  <div class="col-auto ml-2 d-flex align-items-stretch">
    <button class="btn btn-block btn-outline-danger" id="modal-newrepo-btn-remove-field${n + 1}" aria-label="Remove this field" onclick="removeField('#modal-newrepo-field${n + 1}')">&times;</button>
  </div>
</div>`)
})

const removeField = (f) => {
  $(f).detach();
}

$("#modal-newrepo-btn-submit").click(() => {
  const validity = $
    .map($("#modal-newrepo-body input"), (v) => v.validity.valid)
    .reduce((a, v) => a & v, true)
  if (!validity)
    return

  const keys = $.map($("#modal-newrepo-body .real-key"), (v) => v.textContent)
    .concat($.map($("#modal-newrepo-body .key"), (v) => v.value))
  const values = $.map($("#modal-newrepo-body .value"), (v) => v.value)
  const object = Object.fromEntries(keys.map((_, i) => [keys[i], values[i]]))

  $.ajax({
    method: "POST",
    url: "/repository",
    contentType: "application/json",
    data: JSON.stringify(object)
  }).done(() => {
    $("#modal-newrepo").modal("hide")
    location.reload()
  })
})
