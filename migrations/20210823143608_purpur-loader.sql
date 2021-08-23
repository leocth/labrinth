-- Add migration script here
INSERT INTO loaders (loader, icon) VALUES ('purpur', '<?xml version="1.0" encoding="UTF-8"?><svg viewBox="0 0 91 103" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;" fill="currentColor"><g transform="matrix(0.91,0,0,1.03,0,0)"><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M72.09,13.86L53.18,24.8L72.11,35.72L91.02,24.78L72.09,13.86ZM72.094,19.634L81.025,24.786C81.025,24.786 72.106,29.946 72.106,29.946C72.106,29.946 63.175,24.794 63.175,24.794L72.094,19.634Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M49.87,1.05L30.95,11.99L49.88,22.9L68.8,11.97L49.87,1.05ZM49.873,6.824L58.799,11.973C58.799,11.973 49.877,17.127 49.877,17.127C49.877,17.127 40.951,11.983 40.951,11.983L49.873,6.824Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M27.67,13.89L8.75,24.82L27.68,35.74L46.59,24.81L27.67,13.89ZM27.671,19.664L36.593,24.813C36.593,24.813 27.678,29.966 27.678,29.966C27.678,29.966 18.751,24.817 18.751,24.817L27.671,19.664Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M49.89,26.7L30.97,37.64L49.9,48.55L68.82,37.62L49.89,26.7ZM49.893,32.474L58.819,37.623C58.819,37.623 49.897,42.777 49.897,42.777C49.897,42.777 40.971,37.633 40.971,37.633L49.893,32.474Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M7.11,27.67L7.12,49.53L26.05,60.44L26.04,38.59L7.11,27.67ZM12.114,36.329L21.041,41.478C21.041,41.478 21.046,51.785 21.046,51.785C21.046,51.785 12.119,46.64 12.119,46.64L12.114,36.329Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M29.33,40.49L29.34,62.34L48.27,73.26L48.26,51.41L29.33,40.49ZM34.334,49.149L43.261,54.298C43.261,54.298 43.266,64.601 43.266,64.601C43.266,64.601 34.339,59.452 34.339,59.452L34.334,49.149Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M7.12,53.32L7.13,75.17L26.06,86.08L26.05,64.23L7.12,53.32ZM12.124,61.975L21.051,67.12C21.051,67.12 21.056,77.425 21.056,77.425C21.056,77.425 12.129,72.28 12.129,72.28L12.124,61.975Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M29.34,66.14L29.35,87.99L48.28,98.9L48.27,77.05L29.34,66.14ZM34.344,74.795L43.271,79.94C43.271,79.94 43.276,90.245 43.276,90.245C43.276,90.245 34.349,85.1 34.349,85.1L34.344,74.795Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M51.57,98.9L70.49,87.97L70.48,66.12L51.56,77.05L51.57,98.9ZM56.566,90.239L56.562,79.935C56.562,79.935 65.484,74.781 65.484,74.781C65.484,74.781 65.488,85.085 65.488,85.085L56.566,90.239Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M73.78,86.06L92.7,75.13L92.68,53.28L73.77,64.21L73.78,86.06ZM78.776,77.399L78.772,67.094C78.772,67.094 87.688,61.941 87.688,61.941C87.688,61.941 87.697,72.246 87.697,72.246L78.776,77.399Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M73.76,60.41L92.68,49.48L92.67,27.63L73.75,38.56L73.76,60.41ZM78.756,51.749L78.752,41.445C78.752,41.445 87.674,36.291 87.674,36.291C87.674,36.291 87.678,46.595 87.678,46.595L78.756,51.749Z"/></g><g transform="matrix(1.15552,0,0,1.02098,-7.66631,-1.07203)"><path d="M51.55,51.4L51.56,73.25L70.47,62.32L70.46,40.47L51.55,51.4ZM56.552,54.284L65.464,49.133C65.464,49.133 65.468,59.436 65.468,59.436C65.468,59.436 56.556,64.587 56.556,64.587L56.552,54.284Z"/></g></g></svg>') ON CONFLICT DO NOTHING;