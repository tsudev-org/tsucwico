// Hiện dần các khối [data-hien] khi cuộn tới (PRODUCT_POLICY mục 5). Không có JS hoặc người dùng chọn giảm chuyển động
// => không làm gì, nội dung hiện sẵn. Khối đã nằm trong khung nhìn lúc tải không bị ẩn (tránh nháy).
(function () {
  'use strict';
  try {
    if (!('IntersectionObserver' in window)) return;
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  } catch (e) { return; }
  var khoi = document.querySelectorAll('[data-hien]');
  var cao = window.innerHeight || 0;
  var io = new IntersectionObserver(function (muc) {
    muc.forEach(function (m) {
      if (!m.isIntersecting) return;
      m.target.classList.add('hien-ra');
      m.target.classList.remove('hien-cho');
      io.unobserve(m.target);
    });
  }, { rootMargin: '0px 0px -8% 0px' });
  Array.prototype.forEach.call(khoi, function (k) {
    if (k.getBoundingClientRect().top < cao) return;
    k.classList.add('hien-cho');
    io.observe(k);
  });
})();
