CRATE_DIRS  := algorithms_buffer_utils \
			   algorithms_core \
			   algorithms_hash \
			   algorithms_random \
			   algorithms_sort

BENCH_CRATE := benchmark

.PHONY: test
test:
	@DIR=$$(pwd); \
	for dir in $(CRATE_DIRS); do \
		cd "$${DIR}/$${dir}"; \
		cargo test; \
	done;

.PHONY: bench
bench: 
	@cd "${BENCH_CRATE}"; \
		cargo bench;

.PHONY: clean
clean:
	@echo "Cleaning crate directories..."
	@DIR=$$(pwd); \
	for dir in $(CRATE_DIRS); do \
		cd "$${DIR}/$${dir}"; \
		cargo clean; \
	done;
	@echo "Cleaning benchmarks..."
	@cd "$(BENCH_CRATE)"; \
		cargo clean;



