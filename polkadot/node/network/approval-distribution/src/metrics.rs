// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use polkadot_node_metrics::metrics::{prometheus, Metrics as MetricsTrait};
use polkadot_node_primitives::approval::v2::AssignmentCertKindV2;

/// Approval Distribution metrics.
#[derive(Default, Clone)]
pub struct Metrics(Option<MetricsInner>);

#[derive(Clone)]
struct MetricsInner {
	assignments_imported_total: prometheus::CounterVec<prometheus::U64>,
	approvals_imported_total: prometheus::Counter<prometheus::U64>,
	unified_with_peer_total: prometheus::Counter<prometheus::U64>,
	aggression_l1_messages_total: prometheus::Counter<prometheus::U64>,
	aggression_l2_messages_total: prometheus::Counter<prometheus::U64>,
	time_unify_with_peer: prometheus::Histogram,
	time_import_pending_now_known: prometheus::Histogram,
	assignments_received_result: prometheus::CounterVec<prometheus::U64>,
	approvals_received_result: prometheus::CounterVec<prometheus::U64>,
}

trait AsLabel {
	fn as_label(&self) -> &str;
}

impl AsLabel for &AssignmentCertKindV2 {
	fn as_label(&self) -> &str {
		match self {
			AssignmentCertKindV2::RelayVRFDelay { .. } => "VRF Delay",
			AssignmentCertKindV2::RelayVRFModulo { .. } => "VRF Modulo",
			AssignmentCertKindV2::RelayVRFModuloCompact { .. } => "VRF Modulo Compact",
		}
	}
}

impl Metrics {
	pub(crate) fn on_assignment_imported(&self, kind: &AssignmentCertKindV2) {
		if let Some(metrics) = &self.0 {
			metrics.assignments_imported_total.with_label_values(&[kind.as_label()]).inc();
		}
	}

	pub(crate) fn on_approval_imported(&self) {
		if let Some(metrics) = &self.0 {
			metrics.approvals_imported_total.inc();
		}
	}

	pub(crate) fn on_unify_with_peer(&self) {
		if let Some(metrics) = &self.0 {
			metrics.unified_with_peer_total.inc();
		}
	}

	pub(crate) fn time_unify_with_peer(&self) -> Option<prometheus::prometheus::HistogramTimer> {
		self.0.as_ref().map(|metrics| metrics.time_unify_with_peer.start_timer())
	}

	pub(crate) fn time_import_pending_now_known(
		&self,
	) -> Option<prometheus::prometheus::HistogramTimer> {
		self.0
			.as_ref()
			.map(|metrics| metrics.time_import_pending_now_known.start_timer())
	}

	pub(crate) fn on_approval_recent_outdated(&self) {
		if let Some(metrics) = &self.0 {
			metrics.approvals_received_result.with_label_values(&["outdated"]).inc()
		}
	}

	pub(crate) fn on_approval_invalid_block(&self) {
		if let Some(metrics) = &self.0 {
			metrics.approvals_received_result.with_label_values(&["invalidblock"]).inc()
		}
	}

	pub(crate) fn on_approval_unknown_assignment(&self) {
		if let Some(metrics) = &self.0 {
			metrics
				.approvals_received_result
				.with_label_values(&["unknownassignment"])
				.inc()
		}
	}

	pub(crate) fn on_approval_duplicate(&self) {
		if let Some(metrics) = &self.0 {
			metrics.approvals_received_result.with_label_values(&["duplicate"]).inc()
		}
	}

	pub(crate) fn on_approval_out_of_view(&self) {
		if let Some(metrics) = &self.0 {
			metrics.approvals_received_result.with_label_values(&["outofview"]).inc()
		}
	}

	pub(crate) fn on_approval_good_known(&self) {
		if let Some(metrics) = &self.0 {
			metrics.approvals_received_result.with_label_values(&["goodknown"]).inc()
		}
	}

	pub(crate) fn on_approval_bad(&self) {
		if let Some(metrics) = &self.0 {
			metrics.approvals_received_result.with_label_values(&["bad"]).inc()
		}
	}

	pub(crate) fn on_approval_bug(&self) {
		if let Some(metrics) = &self.0 {
			metrics.approvals_received_result.with_label_values(&["bug"]).inc()
		}
	}

	pub(crate) fn on_assignment_recent_outdated(&self) {
		if let Some(metrics) = &self.0 {
			metrics.assignments_received_result.with_label_values(&["outdated"]).inc()
		}
	}

	pub(crate) fn on_assignment_invalid_block(&self) {
		if let Some(metrics) = &self.0 {
			metrics.assignments_received_result.with_label_values(&["invalidblock"]).inc()
		}
	}

	pub(crate) fn on_assignment_duplicate(&self) {
		if let Some(metrics) = &self.0 {
			metrics.assignments_received_result.with_label_values(&["duplicate"]).inc()
		}
	}

	pub(crate) fn on_assignment_out_of_view(&self) {
		if let Some(metrics) = &self.0 {
			metrics.assignments_received_result.with_label_values(&["outofview"]).inc()
		}
	}

	pub(crate) fn on_assignment_good_known(&self) {
		if let Some(metrics) = &self.0 {
			metrics.assignments_received_result.with_label_values(&["goodknown"]).inc()
		}
	}

	pub(crate) fn on_assignment_bad(&self) {
		if let Some(metrics) = &self.0 {
			metrics.assignments_received_result.with_label_values(&["bad"]).inc()
		}
	}

	pub(crate) fn on_assignment_far(&self) {
		if let Some(metrics) = &self.0 {
			metrics.assignments_received_result.with_label_values(&["far"]).inc()
		}
	}

	pub(crate) fn on_aggression_l1(&self) {
		if let Some(metrics) = &self.0 {
			metrics.aggression_l1_messages_total.inc();
		}
	}

	pub(crate) fn on_aggression_l2(&self) {
		if let Some(metrics) = &self.0 {
			metrics.aggression_l2_messages_total.inc();
		}
	}
}

impl MetricsTrait for Metrics {
	fn try_register(registry: &prometheus::Registry) -> Result<Self, prometheus::PrometheusError> {
		let metrics = MetricsInner {
			assignments_imported_total: prometheus::register(
				prometheus::CounterVec::new(
					prometheus::Opts::new(
						"polkadot_parachain_assignments_imported_total",
						"Number of valid assignments imported locally or from other peers.",
					),
					&["kind"],
				)?,
				registry,
			)?,
			approvals_imported_total: prometheus::register(
				prometheus::Counter::new(
					"polkadot_parachain_approvals_imported_total",
					"Number of valid approvals imported locally or from other peers.",
				)?,
				registry,
			)?,
			unified_with_peer_total: prometheus::register(
				prometheus::Counter::new(
					"polkadot_parachain_unified_with_peer_total",
					"Number of times `unify_with_peer` is called.",
				)?,
				registry,
			)?,
			aggression_l1_messages_total: prometheus::register(
				prometheus::Counter::new(
					"polkadot_parachain_approval_distribution_aggression_l1_messages_total",
					"Number of messages in approval distribution for which aggression L1 has been triggered",
				)?,
				registry,
			)?,
			aggression_l2_messages_total: prometheus::register(
				prometheus::Counter::new(
					"polkadot_parachain_approval_distribution_aggression_l2_messages_total",
					"Number of messages in approval distribution for which aggression L2 has been triggered",
				)?,
				registry,
			)?,
			time_unify_with_peer: prometheus::register(
				prometheus::Histogram::with_opts(
					prometheus::HistogramOpts::new(
						"polkadot_parachain_time_unify_with_peer",
						"Time spent within fn `unify_with_peer`.",
					)
					.buckets(vec![
						0.000625, 0.00125, 0.0025, 0.005, 0.0075, 0.01, 0.025, 0.05, 0.1, 0.25,
						0.5, 1.0, 2.5, 5.0, 10.0,
					]),
				)?,
				registry,
			)?,
			time_import_pending_now_known: prometheus::register(
				prometheus::Histogram::with_opts(prometheus::HistogramOpts::new(
					"polkadot_parachain_time_import_pending_now_known",
					"Time spent on importing pending assignments and approvals.",
				).buckets(vec![0.0001, 0.0004, 0.0016, 0.0064, 0.0256, 0.1024, 0.4096, 1.6384, 3.2768, 4.9152, 6.5536,]))?,
				registry,
			)?,
			assignments_received_result: prometheus::register(
				prometheus::CounterVec::new(
					prometheus::Opts::new(
						"polkadot_parachain_assignments_received_result",
						"Result of a processed assignment",
					),
					&["status"]
				)?,
				registry,
			)?,
			approvals_received_result: prometheus::register(
				prometheus::CounterVec::new(
					prometheus::Opts::new(
						"polkadot_parachain_approvals_received_result",
						"Result of a processed approval",
					),
					&["status"]
				)?,
				registry,
			)?,
		};
		Ok(Metrics(Some(metrics)))
	}
}
