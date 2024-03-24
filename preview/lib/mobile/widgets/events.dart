import 'package:beelive/common/dao/dao.dart';
import 'package:beelive/common/problem.dart';
import 'package:beelive/mobile/screens/details.dart';
import 'package:flutter/material.dart';

class ProblemsList extends StatelessWidget {
  final Dao dao;

  const ProblemsList({super.key, required this.dao});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: dao.problems().map((p) {
        return ProblemTile(problem: p);
      }).toList(growable: false),
    );
  }
}

class ProblemTile extends StatelessWidget {
  const ProblemTile({
    super.key,
    required this.problem,
  });

  final Problem problem;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      title: Text(problem.title),
      subtitle: Text(problem.summary),
      selected: false,
      leading: Icon(categoryIcon(problem.categories.firstOrNull)),
      dense: true,
      onTap: () => Navigator.push(
        context,
        MaterialPageRoute(
          builder: (context) => Details(problem: problem),
        ),
      ),
    );
  }
}

IconData categoryIcon(final Category? category) {
  return switch (category) {
    Category.parking => Icons.local_parking,
    Category.viability => Icons.traffic,
    Category.publicTransport => Icons.bus_alert,
    Category.accident => Icons.car_crash,
    null => Icons.info,
  };
}
